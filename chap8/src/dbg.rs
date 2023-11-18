use crate::helper::{do_help, DynErr};
use nix::{
    libc::user_regs_struct,
    sys::{
        personality::{self, Persona},
        ptrace,
        wait::{waitid, WaitStatus},
    },
    unistd::{execvp, fork, ForkResult, Pid},
};
use std::ffi::{c_void, CString};

/// デバッガ内の情報
pub struct DbgInfo {
    pid: Pid,
    brk_addr: Option<*mut c_void>, //ブレークポイントのアドレス
    brk_val: i64,                  //ブレークポイントを設定したメモリの元の値
    filename: String,              //実行ファイル
}

/// デバッガ
/// ZDbg<Running>は子プロセスを実行中。
/// ZDbg<NotRunning>は子プロセスは実行していない
pub struct ZDbg<T> {
    info: Box<DbgInfo>,
    _state: T, //Tはサイズを持たないためこれで良い。サイズを持つ場合は std::marker::PhantomData を用いる
}

/// デバッガの状態
pub struct Running; //実行中
pub struct NotRunning; //実行していない

/// デバッガの列挙型表現 Exitの場合、終了
pub enum State {
    Running(ZDbg<Running>),
    NotRunning(ZDbg<NotRunning>),
    Exit,
}

/// Running と NotRunning で共通の実装
impl<T> ZDbg<T> {
    /// ブレークポイントのアドレスを設定する関数。子プロセスのメモリ上に反映しない
    /// アドレス設定に成功した場合はtrueを返す
    fn set_break_addr(&mut self, cmd: &[&str]) -> bool {
        if self.info.brk_addr.is_some() {
            eprintln!(
                "<< ブレークポイントは設定済みです: Addr = {:p}>>",
                self.info.brk_addr.unwrap()
            );
            false
        } else if let Some(addr) = get_break_addr(cmd) {
            self.info.brk_addr = Some(addr); //ブレークポイントのアドレスを保存
            true
        } else {
            false
        }
    }

    /// 共通のコマンド
    fn do_cmd_common(&self, cmd: &[&str]) {
        match cmd[0] {
            "help" | "h" => do_help(),
            _ => (),
        }
    }
}

/// NotRunning 時に呼び出し可能なメソッド
impl ZDbg<NotRunning> {}

/// Running 時に呼び出し可能なメソッド
impl ZDbg<Running> {
    fn do_step(self) -> Result<State, DynErr> {
        todo!()
    }
}
