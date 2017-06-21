
### 「9pを用いたUserspace filesystem」
#### 静大情報LT大会 -2017年06月-
#### 於 情報学部2号館 DR教室
---

# 自己紹介
## IT悟空
名前: 棟方 亮<br>
所属: 情報社会学科一年, ITS<br>
twitter: [@pf\_packet](https://twitter.com/pf_packet "Twitter")<br>
GitHub: [@pfpacket](https://github.com/pfpacket "GitHub")<br>
--

# 自己紹介
## 趣味
* Arch Linux
* Vim
* 今のお気に入り言語は、Rust
* OpenCVをWaylandに移植
* Waylandを含むOSSにちょいちょいパッチ投稿
* セキュキャン2014 ネットワークセキュリティ
---

# 目次
 ファイルシステム とは
 Userspace filesystem
 FUSE
 9P
 まとめ
 参考文献
---

# ファイルシステム とは
計算機の資源を操作するための、OSが持つ機能の一つ<br>
Unix系OSでは、ファイルは記憶装置上のファイルに留まらず、資源
(tty, プリンタ, ブロックデバイス、キャラクターデバイス等)をファイルとして表現している<br>
---

# Userspace filesystem
カーネル空間だるい<br>
ユーザー空間でファイルシステム組めれば、より安全、簡便<br>
_速度は落ちるが…_
---

# FUSE
_Filesystem in Userspace_<br>
通常のユーザーアプリケーションとして、ファイルシステムを実装出来る
* 既存のコードやライブラリを利用可能
* Cへのバインディングを作成すれば、様々の言語でファイルシステムを作成出来る<br>
_9pというネットワーク透過なUserspace filesystemがあるのに…_
---

# 9P
ベル研の開発した分散OSであるPlan 9のネットワークプロトコル<br>
Plan 9は分散透過でファイルを単なる資源としてだけでなく、サービスとして捉えた。9Pがこれを支えている。<br>
```
例: IP, TCP等のネットワーク、ウィンドウシステム等
```
---

# 9P
## Linuxの対応
Linuxカーネルには9Pのカーネルモジュール(9p, 9pnet)が入っている。<br>
```bash
modinfo 9p
modinfo 9pnet
```
Arch Linuxのレポジトリのカーネルのバイナリでは標準搭載。他ディストリでも同様だと思われる。
カーネルのconfigをいじろう。
--

# 9P
## configの一例
```
CONFIG_9P_FS=m
CONFIG_9P_FSCACHE=y
CONFIG_9P_FS_POSIX_ACL=y
CONFIG_9P_FS_SECURITY=y
CONFIG_NET_9P=m
CONFIG_NET_9P_VIRTIO=m
CONFIG_NET_9P_RDMA=m
CONFIG_NET_9P_DEBUG=y

```
---

# 9P
## ネットワーク透過
簡単に云えば、nfsみたいなこと。<br>
ネットワークを越えてマウント出来て、利用者がそのファイルシステムが本当は何処にあるのか意識する必要が無い。<br>
FUSEと同じく、ファイルシステムはユーザー空間で動く。<br>
---

# わざわざFUSE使うより、9P使ったほうがいいのでは? (迫真)
---

# [rust-9p](https://github.com/pfpacket/rust-9p "rust-9p")
筆者が作成した、Rustで書かれた9Pサーバー用のライブラリ

``` rust:main.rs
struct MyFilesystem;
impl rs9p::Filesystem for MyFilesystem {
    // 此処に実際の処理
    fn rattach(&mut self, fid: &mut Fid<Self::Fid>, _afid: Option<&mut Fid<Self::Fid>>, _uname: &str, _aname: &str, _n_uname: u32) -> Result<Fcall> { unimplemented!(); }
    fn rlopen(&mut self, fid: &mut Fid<Self::Fid>, flags: u32) -> Result<Fcall> { unimplemented!(); }
    fn rread(&mut self, fid: &mut Fid<Self::Fid>, offset: u64, count: u32) -> Result<Fcall> { unimplemented!(); }
    fn rwrite(&mut self, fid: &mut Fid<Self::Fid>, offset: u64, data: &Data) -> Result<Fcall> { unimplemented!(); }
    fn rreaddir(&mut self, fid: &mut Fid<Self::Fid>, off: u64, count: u32) -> Result<Fcall> { unimplemented!(); }
    // ...
}
// 9Pファイルサーバ実行
rs9p::srv_spawn(MyFilesystem, "tcp!0.0.0.0!564");
```
---

# [rust-9p](https://github.com/pfpacket/rust-9p "rust-9p")
## 実行
```
./myfilesystem  # ファイルシステム実行
sudo mount -t 9p -o version=9p2000.L,trans=tcp,port=564,uname=$USER,access=user 127.0.0.1 /n/mysrv2/
```
---

# まとめ
## rust-9pを使えば
* ユーザー空間でファイルシステムを動かす<br>
→安全、容易<br>
* Rustで書ける<br>
→安全<br>
従って、安全かつ容易にファイルシステムが書ける!
---

# 参考文献
[v9fs: Plan 9 Resource Sharing for Linux](https://www.kernel.org/doc/Documentation/filesystems/9p.txt "v9fs doc")

安全になんでも作れる! やってみよう!
