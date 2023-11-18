/// ヘルプを表示
pub fn do_help() {
    println!(
        r#"
break   0x8000  : ブレークポイントを 0x8000 番地に設定 (b 0x8000)
run             : プログラムを実行 (r)
continue        : プログラムを再開 (c)
stepi           : 機械語レベルで1ステップ実行 (s)
registers       : レジスタを表示 (regs)
exit            : 終了
help            : このヘルプを表示 (h)"#
    )
}
