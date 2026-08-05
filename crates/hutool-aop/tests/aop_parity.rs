//! Hutool `hutool-aop` TEST parity —— 对齐 Java `cn.hutool.aop.test.*`。
//!
//! 对齐: `cn.hutool.aop.test.AopTest`
//! 对齐: `cn.hutool.aop.test.IssueI74EX7Test`
//! 对齐: `cn.hutool.aop.test.IssueIBF20ZTest`
//! 来源:
//! - hutool-aop/src/test/java/cn/hutool/aop/test/AopTest.java
//! - hutool-aop/src/test/java/cn/hutool/aop/test/IssueI74EX7Test.java
//! - hutool-aop/src/test/java/cn/hutool/aop/test/IssueIBF20ZTest.java
//!
//! ## CGLIB / Spring CGLIB 说明
//!
//! Rust 无 JVM 字节码子类代理。hutool-aop 用显式 `ProxyBackend::{Cglib,SpringCglib}`
//! 包装已构造的 target，保留 Hutool before/after 回调顺序；不模拟 Enhancer 无参构造。

use hutool_aop::{
    Method, ProxyFactory, ProxyUtil,
    aspects::{SimpleAspect, TimeIntervalAspect},
    proxy::{CglibProxyFactory, JdkProxyFactory, ProxyBackend, SpringCglibProxyFactory},
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

// ---------------------------------------------------------------------------
// Fixtures —— 对齐 AopTest / IssueI74EX7Test 中的内部类
// ---------------------------------------------------------------------------

/// 对齐 Java `AopTest.Animal`
trait Animal {
    /// 对齐 Java `Animal.eat()`
    fn eat(&self) -> String;
    /// 对齐 Java `Animal.seize()`
    fn seize(&mut self);
}

/// 对齐 Java `AopTest.Cat`（有接口 → JDK 代理）
#[derive(Debug, Default)]
struct Cat;

impl Animal for Cat {
    fn eat(&self) -> String {
        "猫吃鱼".to_owned()
    }

    fn seize(&mut self) {
        // Java: Console.log("抓了条鱼") —— 副作用无断言
    }
}

/// 对齐 Java `AopTest.Dog`（无接口 → CGLIB 代理）
#[derive(Debug, Default)]
struct Dog;

impl Dog {
    /// 对齐 Java `Dog.eat()`
    fn eat(&self) -> String {
        "狗吃肉".to_owned()
    }

    /// 对齐 Java `Dog.seize()`
    fn seize(&mut self) {
        // Java: Console.log("抓了只鸡") —— 副作用无断言
    }
}

/// 对齐 Java `AopTest.TagObj`
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct TagObj {
    tag: Option<String>,
}

impl TagObj {
    /// 对齐 Java Lombok `@Data` setter
    fn set_tag(&mut self, tag: impl Into<String>) {
        self.tag = Some(tag.into());
    }

    /// 对齐 Java Lombok `@Data` getter
    fn get_tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }
}

/// 对齐 Java `IssueI74EX7Test.SmsBlend`
trait SmsBlend {
    /// 对齐 Java `SmsBlend.send()`
    fn send(&self) -> String;
}

/// 对齐 Java `IssueI74EX7Test.SmsBlendImpl`（有参构造）
#[derive(Debug)]
struct SmsBlendImpl {
    status: i32,
}

impl SmsBlendImpl {
    /// 对齐 Java `SmsBlendImpl(int status)`
    fn new(status: i32) -> Self {
        Self { status }
    }
}

impl SmsBlend for SmsBlendImpl {
    fn send(&self) -> String {
        format!("sms send.{}", self.status)
    }
}

/// 对齐 Java `IssueI74EX7Test.SmsBlendImplWithoutConstructor`
#[derive(Debug, Default)]
struct SmsBlendImplWithoutConstructor {
    status: i32,
}

impl SmsBlend for SmsBlendImplWithoutConstructor {
    fn send(&self) -> String {
        format!("sms send.{}", self.status)
    }
}

// ---------------------------------------------------------------------------
// Existing smoke test (保留，勿删)
// ---------------------------------------------------------------------------

#[test]
fn interceptor_chain_test() {
    let chain = hutool_aop::InterceptorChain::<i32, String, String>::new();
    assert!(chain.is_empty(), "空 chain 应为空");
}

// ---------------------------------------------------------------------------
// AopTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `AopTest.aopTest()`
///
/// Java: `ProxyUtil.proxy(new Cat(), TimeIntervalAspect.class)`，
/// `assertEquals("猫吃鱼", cat.eat())`，再调用 `cat.seize()`。
#[test]
fn aop_test() {
    let mut cat = ProxyUtil::proxy(Cat, TimeIntervalAspect::new());
    let result = cat
        .invoke(&Method::new("eat"), &mut (), |target, _| {
            Ok::<_, ()>(target.eat())
        })
        .expect("eat through JDK-style proxy");
    assert_eq!(result, Some("猫吃鱼".to_owned()));

    let seize = cat
        .invoke(&Method::new("seize"), &mut (), |target, _| {
            target.seize();
            Ok::<_, ()>(String::new())
        })
        .expect("seize through JDK-style proxy");
    assert_eq!(seize, Some(String::new()));
}

/// 对齐 Java: `AopTest.aopByAutoCglibTest()`
///
/// Java: `ProxyUtil.proxy(new Dog(), TimeIntervalAspect.class)` 因无接口自动选 CGLIB。
/// Rust 无字节码增强：显式 `CglibProxyFactory`，断言与 Java 相同的 `eat()` 返回值。
#[test]
fn aop_by_auto_cglib_test() {
    // Divergence: JVM auto-picks CGLIB for class-without-interface;
    // Rust uses explicit CglibProxyFactory while preserving TimeIntervalAspect callbacks.
    let mut dog = CglibProxyFactory::proxy(Dog, TimeIntervalAspect::new());
    let result = dog
        .invoke(&Method::new("eat"), &mut (), |target, _| {
            Ok::<_, ()>(target.eat())
        })
        .expect("eat through CGLIB-style proxy");
    assert_eq!(result, Some("狗吃肉".to_owned()));

    let seize = dog
        .invoke(&Method::new("seize"), &mut (), |target, _| {
            target.seize();
            Ok::<_, ()>(String::new())
        })
        .expect("seize through CGLIB-style proxy");
    assert_eq!(seize, Some(String::new()));
}

/// 对齐 Java: `AopTest.testCGLIBProxy()`
///
/// Java: `TagObj` 设 `tag="tag"`，`ProxyUtil.proxy` 后 `assertEquals("tag", proxy.getTag())`。
/// Rust: 显式 CGLIB 后端包装已设 tag 的 target，经代理链读取 tag。
#[test]
fn test_cglib_proxy() {
    let mut target = TagObj::default();
    target.set_tag("tag");

    // Divergence: Java CGLIB subclass retains field state on enhancer instance;
    // Rust wraps the moved target — getTag via invoke must still yield "tag".
    let mut proxy = CglibProxyFactory::proxy(target, TimeIntervalAspect::new());
    let tag = proxy
        .invoke(&Method::new("getTag"), &mut (), |obj, _| {
            Ok::<_, ()>(obj.get_tag().map(str::to_owned))
        })
        .expect("getTag through CGLIB-style proxy");
    assert_eq!(tag, Some(Some("tag".to_owned())));
    assert_eq!(proxy.get_target().get_tag(), Some("tag"));
}

// ---------------------------------------------------------------------------
// IssueI74EX7Test
// ---------------------------------------------------------------------------

/// 对齐 Java: `IssueI74EX7Test.proxyTest()`
///
/// Java: `new JdkProxyFactory().proxy(new SmsBlendImpl(1), new SimpleAspect())`（无断言，不抛即可）。
#[test]
fn proxy_test() {
    let sms = SmsBlendImpl::new(1);
    let mut proxy = JdkProxyFactory::proxy(sms, SimpleAspect);
    let out = proxy
        .invoke(&Method::new("send"), &mut (), |target, _| {
            Ok::<_, ()>(target.send())
        })
        .expect("JDK proxy send");
    assert_eq!(out, Some("sms send.1".to_owned()));
}

/// 对齐 Java: `IssueI74EX7Test.cglibProxyTest()`
///
/// Java: `new CglibProxyFactory().proxy(new SmsBlendImpl(1), new SimpleAspect())`。
/// Issue I74EX7 关注 CGLIB Enhancer 对有参构造的兼容；Rust 包装已构造实例，行为对等可用。
#[test]
fn cglib_proxy_test() {
    // Divergence: no Enhancer.create() / no-arg ctor — wrap constructed SmsBlendImpl(1).
    let sms = SmsBlendImpl::new(1);
    let mut proxy = CglibProxyFactory::proxy(sms, SimpleAspect);
    let out = proxy
        .invoke(&Method::new("send"), &mut (), |target, _| {
            Ok::<_, ()>(target.send())
        })
        .expect("CGLIB proxy send");
    assert_eq!(out, Some("sms send.1".to_owned()));
}

/// 对齐 Java: `IssueI74EX7Test.springCglibProxyTest()`
///
/// Java: `new SpringCglibProxyFactory().proxy(new SmsBlendImpl(1), new SimpleAspect())`。
#[test]
fn spring_cglib_proxy_test() {
    // Divergence: Spring CGLIB Enhancer ctor path → explicit SpringCglib backend.
    let sms = SmsBlendImpl::new(1);
    let mut proxy = SpringCglibProxyFactory::proxy(sms, SimpleAspect);
    let out = proxy
        .invoke(&Method::new("send"), &mut (), |target, _| {
            Ok::<_, ()>(target.send())
        })
        .expect("Spring-CGLIB proxy send");
    assert_eq!(out, Some("sms send.1".to_owned()));
}

/// 对齐 Java: `IssueI74EX7Test.springCglibProxyWithoutConstructorTest()`
///
/// Java: `new SpringCglibProxyFactory().proxy(new SmsBlendImplWithoutConstructor(), ...)`。
#[test]
fn spring_cglib_proxy_without_constructor_test() {
    let sms = SmsBlendImplWithoutConstructor::default();
    let mut proxy = SpringCglibProxyFactory::proxy(sms, SimpleAspect);
    let out = proxy
        .invoke(&Method::new("send"), &mut (), |target, _| {
            Ok::<_, ()>(target.send())
        })
        .expect("Spring-CGLIB proxy send (default ctor)");
    assert_eq!(out, Some("sms send.0".to_owned()));
}

// ---------------------------------------------------------------------------
// IssueIBF20ZTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `IssueIBF20ZTest.testLoadFirstAvailableConcurrent()`
///
/// Java: 1000 线程并发 `ProxyFactory.create()`，断言全部非 null。
/// Rust: `ProxyFactory::create()` 为纯值构造，同样并发 1000 次并断言全部成功。
#[test]
fn test_load_first_available_concurrent() {
    let thread_count = 1000usize;
    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(thread_count);

    for _ in 0..thread_count {
        let counter = Arc::clone(&success_count);
        handles.push(std::thread::spawn(move || {
            let factory = ProxyFactory::create();
            // Java: factory != null；Rust 工厂始终可构造且默认 JDK 后端
            if factory.backend() == ProxyBackend::Jdk {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for handle in handles {
        handle.join().expect("worker thread");
    }

    assert_eq!(success_count.load(Ordering::SeqCst), thread_count);
}
