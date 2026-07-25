//! 对齐: Hutool AOP 模块端到端测试
//! 来源: hutool-aop/src/test/java/cn/hutool/aop/
//! 中文说明: AOP 模块的端到端测试，验证代理、拦截器、切面及工厂的对齐语义。

use crate::{
    BeforeAfter, HandlerProxy, Interceptor, InterceptorChain, InvocationHandler, Method,
    aspects::{Aspect, SimpleAspect, TimeIntervalAspect, TimingEvent},
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
    proxy::{
        CglibProxyFactory, JdkProxyFactory, Proxy, ProxyBackend, ProxyFactory, ProxyUtil,
        SpringCglibProxyFactory,
    },
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, PartialEq, Eq)]
struct Counter(i32);

#[derive(Clone)]
struct RecordingAspect {
    events: Arc<Mutex<Vec<&'static str>>>,
    allow_before: bool,
    allow_after: bool,
    propagate_error: bool,
}

impl RecordingAspect {
    fn allowing(events: Arc<Mutex<Vec<&'static str>>>) -> Self {
        Self {
            events,
            allow_before: true,
            allow_after: true,
            propagate_error: true,
        }
    }

    fn push(&self, event: &'static str) {
        self.events.lock().unwrap().push(event);
    }
}

impl Aspect<Counter, i32, i32, &'static str> for RecordingAspect {
    fn before(&self, _target: &Counter, method: &Method, _args: &i32) -> bool {
        assert_eq!(method.name(), "add");
        self.push("before");
        self.allow_before
    }

    fn after(
        &self,
        _target: &Counter,
        _method: &Method,
        _args: &i32,
        return_value: Option<&i32>,
    ) -> bool {
        self.push(if return_value.is_some() {
            "after-value"
        } else {
            "after-none"
        });
        self.allow_after
    }

    fn after_exception(
        &self,
        _target: &Counter,
        _method: &Method,
        _args: &i32,
        _error: &&'static str,
    ) -> bool {
        self.push("error");
        self.propagate_error
    }
}

fn operate(target: &mut Counter, amount: &mut i32) -> Result<i32, &'static str> {
    if *amount == i32::MIN {
        return Err("boom");
    }
    target.0 += *amount;
    Ok(target.0)
}

fn events(events: &Arc<Mutex<Vec<&'static str>>>) -> Vec<&'static str> {
    events.lock().unwrap().clone()
}

#[test]
fn methods_and_handler_proxies_are_typed_and_explicit() {
    let method = Method::new("add");
    assert_eq!(method.name(), "add");
    assert_eq!(method.clone(), Method::new(String::from("add")));

    let handler =
        |target: &mut Counter, called: &Method, amount: &mut i32| -> Result<i32, &'static str> {
            assert_eq!(called, &method);
            operate(target, amount)
        };
    let mut proxy = HandlerProxy::new(Counter::default(), handler);
    assert_eq!(proxy.get_target(), &Counter(0));
    proxy.get_target_mut().0 = 1;
    assert!(format!("{proxy:?}").contains("HandlerProxy"));
    assert_eq!(proxy.invoke(&method, &mut 2), Ok(3));
    assert_eq!(proxy.into_target(), Counter(3));

    let mut facade = ProxyUtil::new_proxy_instance(
        Counter::default(),
        |target: &mut Counter, _: &Method, amount: &mut i32| operate(target, amount),
    );
    assert_eq!(facade.invoke(&method, &mut 4), Ok(4));

    let loader = "application";
    let mut loaded_proxy = ProxyUtil::new_proxy_instance_with_loader(
        Counter::default(),
        |target: &mut Counter, _: &Method, amount: &mut i32| operate(target, amount),
        &loader,
    );
    assert_eq!(loaded_proxy.invoke(&method, &mut 5), Ok(5));
}

#[test]
fn invocation_handler_trait_dispatches_closures() {
    let handler = |target: &mut Counter,
                   _method: &Method,
                   amount: &mut i32|
     -> Result<i32, &'static str> { operate(target, amount) };
    let mut target = Counter::default();
    assert_eq!(
        InvocationHandler::invoke(&handler, &mut target, &Method::new("add"), &mut 3),
        Ok(3)
    );
}

#[test]
fn simple_and_timing_aspects_preserve_hutool_defaults() {
    let method = Method::new("add");
    let simple = SimpleAspect;
    assert!(
        <SimpleAspect as Aspect<Counter, i32, i32, &'static str>>::before(
            &simple,
            &Counter(0),
            &method,
            &1
        )
    );
    assert!(
        <SimpleAspect as Aspect<Counter, i32, i32, &'static str>>::after(
            &simple,
            &Counter(0),
            &method,
            &1,
            Some(&1)
        )
    );
    assert!(
        <SimpleAspect as Aspect<Counter, i32, i32, &'static str>>::after_exception(
            &simple,
            &Counter(0),
            &method,
            &1,
            &"boom"
        )
    );

    let captured = Arc::new(Mutex::new(Vec::<TimingEvent>::new()));
    let sink = Arc::clone(&captured);
    let timing = TimeIntervalAspect::with_sink(move |event| {
        sink.lock().unwrap().push(event.clone());
    });
    assert!(
        <TimeIntervalAspect as Aspect<Counter, i32, i32, ()>>::before(
            &timing,
            &Counter(0),
            &method,
            &1
        )
    );
    assert!(
        <TimeIntervalAspect as Aspect<Counter, i32, i32, ()>>::after(
            &timing,
            &Counter(1),
            &method,
            &1,
            Some(&1)
        )
    );
    let recorded = captured.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert!(recorded[0].target_type.ends_with("Counter"));
    assert_eq!(recorded[0].method, "add");
    assert_eq!(recorded[0].return_value.as_deref(), Some("1"));
    assert_eq!(timing.last_elapsed(), recorded[0].elapsed);
    assert!(format!("{timing:?}").contains("TimeIntervalAspect"));
    drop(recorded);

    let no_sink = TimeIntervalAspect::new();
    assert!(
        <TimeIntervalAspect as Aspect<Counter, i32, i32, ()>>::after(
            &no_sink,
            &Counter(0),
            &method,
            &0,
            None
        )
    );
    assert_eq!(no_sink.last_elapsed(), std::time::Duration::ZERO);
}

#[test]
fn timing_aspect_keeps_nested_invocations_independent() {
    let timing = TimeIntervalAspect::new();
    let method = Method::new("nested");
    assert!(<TimeIntervalAspect as Aspect<(), (), i32, ()>>::before(
        &timing,
        &(),
        &method,
        &()
    ));
    assert!(<TimeIntervalAspect as Aspect<(), (), i32, ()>>::before(
        &timing,
        &(),
        &method,
        &()
    ));
    assert!(<TimeIntervalAspect as Aspect<(), (), i32, ()>>::after(
        &timing,
        &(),
        &method,
        &(),
        Some(&2)
    ));
    assert!(<TimeIntervalAspect as Aspect<(), (), i32, ()>>::after(
        &timing,
        &(),
        &method,
        &(),
        Some(&3)
    ));
}

#[test]
fn jdk_interceptor_matches_callback_and_suppression_semantics() {
    let method = Method::new("add");

    let log = Arc::new(Mutex::new(Vec::new()));
    let mut success =
        JdkInterceptor::new(Counter::default(), RecordingAspect::allowing(log.clone()));
    assert_eq!(success.get_target(), &Counter(0));
    success.get_target_mut().0 = 1;
    assert!(format!("{success:?}").contains("JdkInterceptor"));
    assert_eq!(success.invoke(&method, &mut 2, operate), Ok(Some(3)));
    assert_eq!(events(&log), ["before", "after-value"]);
    assert_eq!(success.into_target(), Counter(3));

    let log = Arc::new(Mutex::new(Vec::new()));
    let mut blocked = RecordingAspect::allowing(log.clone());
    blocked.allow_before = false;
    let mut interceptor = JdkInterceptor::new(Counter::default(), blocked);
    assert_eq!(interceptor.invoke(&method, &mut 2, operate), Ok(None));
    assert_eq!(events(&log), ["before"]);

    let log = Arc::new(Mutex::new(Vec::new()));
    let mut hides_value = RecordingAspect::allowing(log.clone());
    hides_value.allow_after = false;
    let mut interceptor = JdkInterceptor::with_shared(Counter::default(), Arc::new(hides_value));
    assert_eq!(interceptor.invoke(&method, &mut 2, operate), Ok(None));
    assert_eq!(events(&log), ["before", "after-value"]);

    let log = Arc::new(Mutex::new(Vec::new()));
    let mut interceptor =
        JdkInterceptor::new(Counter::default(), RecordingAspect::allowing(log.clone()));
    let mut error_argument = i32::MIN;
    assert_eq!(
        interceptor.invoke(&method, &mut error_argument, operate),
        Err("boom")
    );
    assert_eq!(events(&log), ["before", "error"]);

    let log = Arc::new(Mutex::new(Vec::new()));
    let mut suppresses = RecordingAspect::allowing(log.clone());
    suppresses.propagate_error = false;
    let mut interceptor = JdkInterceptor::new(Counter::default(), suppresses);
    let mut error_argument = i32::MIN;
    assert_eq!(
        interceptor.invoke(&method, &mut error_argument, operate),
        Ok(None)
    );
    assert_eq!(events(&log), ["before", "error", "after-none"]);
}

#[test]
fn cglib_variants_run_after_when_before_rejects() {
    let method = Method::new("add");
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut blocked = RecordingAspect::allowing(log.clone());
    blocked.allow_before = false;
    let mut cglib = CglibInterceptor::new(Counter::default(), blocked);
    assert_eq!(cglib.get_target(), &Counter(0));
    cglib.get_target_mut().0 = 1;
    assert!(format!("{cglib:?}").contains("CglibInterceptor"));
    assert_eq!(cglib.invoke(&method, &mut 2, operate), Ok(None));
    assert_eq!(events(&log), ["before", "after-none"]);
    assert_eq!(cglib.into_target(), Counter(1));

    let log = Arc::new(Mutex::new(Vec::new()));
    let shared: Arc<dyn Aspect<Counter, i32, i32, &'static str>> =
        Arc::new(RecordingAspect::allowing(log.clone()));
    let mut spring = SpringCglibInterceptor::with_shared(Counter::default(), shared);
    assert_eq!(spring.get_target(), &Counter(0));
    spring.get_target_mut().0 = 2;
    assert!(format!("{spring:?}").contains("SpringCglibInterceptor"));
    assert_eq!(spring.invoke(&method, &mut 3, operate), Ok(Some(5)));
    assert_eq!(spring.into_target(), Counter(5));
    assert_eq!(events(&log), ["before", "after-value"]);

    let mut owned = SpringCglibInterceptor::new(Counter::default(), SimpleAspect);
    assert_eq!(owned.invoke(&method, &mut 1, operate), Ok(Some(1)));
}

fn exercise_proxy(mut proxy: Proxy<Counter, i32, i32, &'static str>) -> Counter {
    assert_eq!(proxy.get_target(), &Counter(0));
    proxy.get_target_mut().0 = 1;
    assert!(!format!("{proxy:?}").is_empty());
    assert_eq!(
        proxy.invoke(&Method::new("add"), &mut 2, operate),
        Ok(Some(3))
    );
    proxy.into_target()
}

#[test]
fn factories_select_all_backends_and_facade_overloads() {
    let factory = ProxyFactory::create();
    assert_eq!(factory.backend(), ProxyBackend::Jdk);
    assert_eq!(
        exercise_proxy(factory.proxy(Counter::default(), SimpleAspect)),
        Counter(3)
    );
    assert_eq!(
        exercise_proxy(
            ProxyFactory::with_backend(ProxyBackend::Cglib)
                .proxy_default::<Counter, i32, i32, &'static str, SimpleAspect>(Counter::default())
        ),
        Counter(3)
    );

    let shared: Arc<dyn Aspect<Counter, i32, i32, &'static str>> = Arc::new(SimpleAspect);
    assert_eq!(
        exercise_proxy(
            ProxyFactory::with_backend(ProxyBackend::SpringCglib)
                .proxy_shared(Counter::default(), shared)
        ),
        Counter(3)
    );
    assert_eq!(
        exercise_proxy(ProxyFactory::create_proxy(Counter::default(), SimpleAspect)),
        Counter(3)
    );
    assert_eq!(
        exercise_proxy(ProxyFactory::create_proxy_default::<
            Counter,
            i32,
            i32,
            &'static str,
            SimpleAspect,
        >(Counter::default())),
        Counter(3)
    );
    assert_eq!(
        exercise_proxy(JdkProxyFactory::proxy(Counter::default(), SimpleAspect)),
        Counter(3)
    );
    assert_eq!(
        exercise_proxy(CglibProxyFactory::proxy(Counter::default(), SimpleAspect)),
        Counter(3)
    );
    assert_eq!(
        exercise_proxy(SpringCglibProxyFactory::proxy(
            Counter::default(),
            SimpleAspect
        )),
        Counter(3)
    );
    assert_eq!(
        exercise_proxy(ProxyUtil::proxy(Counter::default(), SimpleAspect)),
        Counter(3)
    );
    assert_eq!(
        exercise_proxy(ProxyUtil::proxy_default::<
            Counter,
            i32,
            i32,
            &'static str,
            SimpleAspect,
        >(Counter::default())),
        Counter(3)
    );
}

struct SharedInterceptor;

impl Interceptor<Vec<&'static str>, usize, &'static str> for SharedInterceptor {
    fn intercept(
        &self,
        context: &mut Vec<&'static str>,
        next: &mut dyn FnMut(&mut Vec<&'static str>) -> Result<usize, &'static str>,
    ) -> Result<usize, &'static str> {
        context.push("shared-before");
        let result = next(context);
        context.push("shared-after");
        result
    }
}

#[test]
fn generic_interceptor_chain_covers_empty_shared_and_error_paths() {
    let empty = InterceptorChain::<Vec<&'static str>, usize, &'static str>::default();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.execute(&mut Vec::new(), |_| Ok(7)), Ok(7));

    let chain = InterceptorChain::new()
        .with(BeforeAfter::new(
            |events: &mut Vec<&'static str>| events.push("before"),
            |events: &mut Vec<&'static str>, result: &Result<usize, &'static str>| {
                events.push(if result.is_ok() { "ok" } else { "error" });
            },
        ))
        .with_shared(Arc::new(SharedInterceptor));
    assert!(!chain.is_empty());
    assert_eq!(chain.len(), 2);
    assert!(format!("{chain:?}").contains("interceptor_count"));
    let mut events = Vec::new();
    assert_eq!(chain.execute(&mut events, |_| Err("failed")), Err("failed"));
    assert_eq!(events, ["before", "shared-before", "shared-after", "error"]);
}
