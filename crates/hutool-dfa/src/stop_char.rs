//! 对齐: `cn.hutool.dfa.StopChar`
//! 来源: hutool-dfa/src/main/java/cn/hutool/dfa/StopChar.java
//! 中文说明: 停用字符分类，用于敏感词匹配时跳过无意义字符

/// Stop-character predicates used by [`crate::WordTree`].
#[derive(Debug, Default, Clone, Copy)]
pub struct StopChar;

impl StopChar {
    /// Returns whether `character` is ignored during matching.
    #[must_use]
    pub fn is_stop_char(character: char) -> bool {
        character.is_whitespace() || STOP_CHARS.contains(character)
    }

    /// Returns whether `character` participates in matching.
    #[must_use]
    pub fn is_not_stop_char(character: char) -> bool {
        !Self::is_stop_char(character)
    }
}

const STOP_CHARS: &str = r#"'、。·ˉˇ々—～‖…‘’“”〔〕〈〉《》「」『』〖〗【】±＋－×÷∧∨∑∏∪∩∈√⊥⊙∫∮≡≌≈∽∝≠≮≯≤≥∞∶∵∴∷♂♀°′〃℃＄¤￠￡‰§☆★〇○●◎◇◆□■△▽⊿▲▼◣◤◢◥▁▂▃▄▅▆▇█▉▊▋▌▍▎▏▓※→←↑↓↖↗↘↙〓ⅰⅱⅲⅳⅴⅵⅶⅷⅸⅹ①②③④⑤⑥⑦⑧⑨⑩⒈⒉⒊⒋⒌⒍⒎⒏⒐⒑⒒⒓⒔⒕⒖⒗⒘⒙⒚⒛⑴⑵⑶⑷⑸⑹⑺⑻⑼⑽⑾⑿⒀⒁⒂⒃⒄⒅⒆⒇ⅠⅡⅢⅣⅤⅥⅦⅧⅨⅩⅪⅫ！＃￥％＆（）＊，．／０１２３４５６７８９：；＜＝＞？＠〔＼〕＾＿‘｛｜｝ΡΥΦΧΨΩαβγδεζηθικλμνξοπρστυφχψω﹊﹍╭╮╰╯_^：！/\"<>`{}~()-$@*&#卐㎎㎏㎜㎝㎞㎡㏄㏎㏑㏒㏕+=?:.!;]|%"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_whitespace_hutool_symbols_and_normal_text() {
        assert!(StopChar::is_stop_char('\n'));
        assert!(StopChar::is_stop_char('。'));
        assert!(StopChar::is_stop_char('０'));
        assert!(!StopChar::is_stop_char('敏'));
        assert!(StopChar::is_not_stop_char('A'));
        assert!(!StopChar::is_not_stop_char('-'));
    }
}
