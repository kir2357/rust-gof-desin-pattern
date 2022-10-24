# Rust Design Pattern Practice

## 目的

## 参考サイト

- 参考サイト１：[Rustで写経、デザインパターン23種 - Qiita](https://qiita.com/mopp/items/3794dc955f7dc9d8ca63#proxy-%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3)
- 参考サイト２：[Rust風にデザインパターン23種 | κeenのHappy Hacκing Blog](https://keens.github.io/blog/2017/05/06/rustkazenidezainpata_n23tane/)
- 参考サイト３：[デザインパターンを Rust で](https://refactoring.guru/ja/design-patterns/rust)
- [デザインパターン一覧　[23種類] - Qiita](https://qiita.com/ichi-nakashima/items/ee09c9341f85c18f748a)

- [毎朝15分の勉強会で若手の設計力がメキメキアップした話 - Qiita](https://qiita.com/kojimadev/items/99d2aa1c9bc67a835480#37-%E3%82%AF%E3%83%A9%E3%82%B9%E5%9B%B3%E3%81%A0%E3%81%91%E6%9B%B8%E3%81%84%E3%81%A6%E3%82%82%E8%BA%AB%E3%81%AB%E4%BB%98%E3%81%8B%E3%81%AA%E3%81%84)

## 表

<table>
 <tr>
  <td>No</td>
  <td>パターン名</td>
  <td>特徴</td>
  <td>参考サイト１</td>
  <td>参考サイト２</td>
 </tr>
 <tr>
  <td>1</td>
  <td>Iterator</td>
  <td>１つ１つ数え上げる</td>
  <td>振る舞い</td>
  <td>←</td>
 </tr>
 <tr>
  <td>2</td>
  <td>Adapter</td>
  <td>一皮かぶせて再利用</td>
  <td>構造</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>3</td>
  <td>Template Method</td>
  <td>具体的な処理をサブクラスにまかせる</td>
  <td>振る舞い</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>4</td>
  <td>Factory Method</td>
  <td>インスタンス作成をサブクラスにまかせる</td>
  <td>生成</td>
  <td>〇</td>
 </tr>
 <tr>
  <td>5</td>
  <td>Singleton</td>
  <td>たった１つのインスタンス</td>
  <td>生成</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>6</td>
  <td>Prototype</td>
  <td>コピーしてインスタンスを作る</td>
  <td>生成</td>
  <td>←</td>
 </tr>
 <tr>
  <td>7</td>
  <td>Builder</td>
  <td>複雑なインスタンスを組み立てる</td>
  <td>生成</td>
  <td>？</td>
 </tr>
 <tr>
  <td>8</td>
  <td>Abstract Factory</td>
  <td>関連する部品を組み合わせて製品を作る</td>
  <td>生成</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>9</td>
  <td>Bridge</td>
  <td>機能階層と実装の階層を分ける</td>
  <td>構造</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>10</td>
  <td>Strategy</td>
  <td>アルゴリズムをごっそり切り替える</td>
  <td>振る舞い</td>
  <td>reference</td>
 </tr>
 <tr>
  <td>11</td>
  <td>Composite</td>
  <td>容器と中身の同一視</td>
  <td>構造</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>12</td>
  <td>Decorator</td>
  <td>飾り枠と中身の同一視</td>
  <td>構造</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>13</td>
  <td>Visitor</td>
  <td>構造を渡り歩きながら仕事をする</td>
  <td>振る舞い</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>14</td>
  <td>Chain of Responsibility</td>
  <td>責任のたらい回し</td>
  <td>生成</td>
  <td>←</td>
 </tr>
 <tr>
  <td>15</td>
  <td>Facade</td>
  <td>シンプルな窓口</td>
  <td>構造</td>
  <td>←</td>
 </tr>
 <tr>
  <td>16</td>
  <td>Mediator</td>
  <td>相手は相談役一人だけ</td>
  <td>振る舞い</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>17</td>
  <td>Observer</td>
  <td>状態の変化を通知する</td>
  <td>振る舞い</td>
  <td>←</td>
 </tr>
 <tr>
  <td>18</td>
  <td>Memento</td>
  <td>状態を保存する</td>
  <td>振る舞い</td>
  <td>←</td>
 </tr>
 <tr>
  <td>19</td>
  <td>State</td>
  <td>状態をクラスとして表現する</td>
  <td>振る舞い</td>
  <td>←</td>
 </tr>
 <tr>
  <td>20</td>
  <td>Flyweight</td>
  <td>同じものを共有して無駄をなくす</td>
  <td>構造</td>
  <td>appendix</td>
 </tr>
 <tr>
  <td>21</td>
  <td>Proxy</td>
  <td>必要になってから作る</td>
  <td>構造</td>
  <td>←</td>
 </tr>
 <tr>
  <td>22</td>
  <td>Command</td>
  <td>命令をクラスにする</td>
  <td>振る舞い</td>
  <td>〇</td>
 </tr>
 <tr>
  <td>23</td>
  <td>Interpreter</td>
  <td>文法規則をクラスで表現する</td>
  <td>振る舞い</td>
  <td>？</td>
 </tr>
</table>
