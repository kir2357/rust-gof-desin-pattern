// 1.Command(命令)
// 命令のインタフェースを定義します。
trait Command<T> {
    // fn execute(&self, &mut T);
    // fn undo(&self, &mut T);
    fn execute(&self,t:&mut T);
    fn undo(&self,t:&mut T);
}

// 6.Invoker(起動者)
// 「Command」で定義されているインタフェースを呼出します。 
// また、複数の「ConcreteCommand」を保持することにより、命令の履歴管理機能、UNDO機能等を提供します。
struct Invoker<'a, Cmd, T: 'a> {
    commands: Vec<Cmd>,
    // コマンドの列挙体Cmdを作る
    target: &'a mut T,
    current_index: usize,
}


impl<'a, Cmd, T> Invoker<'a, Cmd, T> {
    fn new(t: &'a mut T) -> Self {
        Invoker {
            commands: Vec::new(),
            target: t,
            current_index: 0,
        }
    }


    fn target(&self) -> &T {
        self.target
    }

    fn append_command(&mut self, c: Cmd) {
        self.commands.push(c);
    }
}

impl<'a, Cmd, T> Invoker<'a, Cmd, T>
    where Cmd: Command<T>
{
    fn execute_command(&mut self) {
        if self.commands.len() <= self.current_index {
            // Nothing to do.
            return;
        }

        let c = &self.commands[self.current_index];
        let t = &mut *self.target;
        c.execute(t);

        self.current_index += 1;
    }

    fn execute_all_commands(&mut self) {
        for _ in self.current_index..self.commands.len() {
            self.execute_command();
        }
    }

    fn undo(&mut self) {
        if 0 == self.current_index {
            return;
        }

        self.current_index -= 1;

        let c = &self.commands[self.current_index];
        let t = &mut *self.target;
        c.undo(t);
    }
}


#[derive(Debug, Eq, PartialEq)]
// 3.Receiver(受信者)
// 「Command」の処理対象となるオブジェクトのインタフェースです。

// 4. ConcreteReceiver(具体的な受信者)
// 「Receiver」のインタフェースを実装します。
//  この様に、「Receiver」インタフェースを介することで、
// 複数の命令の受取手(「ConcreteReceiver」)を 作成することができます。

// このコードではRobotオブジェクトにたいして複数の利用者から使われることを想定していない
// ⇒4は実装していない
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}


impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    }

    fn move_forward(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn set_direction(&mut self, d: (i32, i32)) {
        self.dx = d.0;
        self.dy = d.1;
    }

    fn get_direction(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }
}
// 2. ConcreteCommandA・B(具体的な命令)
// 「Command」のインタフェースを実装します。
// コマンドを列挙体に入れる
enum RoboCommand {
    MoveForward,
    TurnRight,
    TurnLeft,
}

// 列挙体にexcuteを実装
// undoは必要であれば
impl Command<Robot> for RoboCommand {
    fn execute(&self, r: &mut Robot) {
        use RoboCommand::*;
        match *self {
            MoveForward => r.move_forward(),
            TurnRight => {
                let (dx, dy) = r.get_direction();
                r.set_direction((dy, -dx))
            }
            TurnLeft => {
                let (dx, dy) = r.get_direction();
                r.set_direction((-dy, dx));
            }

        }
    }
    fn undo(&self, r: &mut Robot) {
        use RoboCommand::*;
        match *self {
            MoveForward => {
                // 反転⇒前進⇒反転
                let c1 = TurnRight;
                c1.execute(r);
                c1.execute(r);
                self.execute(r);
                c1.execute(r);
                c1.execute(r);
            }
            TurnRight => {
                let c = TurnLeft;
                c.execute(r);
            }
            TurnLeft => {
                let c = TurnRight;
                c.execute(r);

            }

        }
    }
}



#[test] 
fn main() {

    // 6. Client(利用者)
    // 「ConcreteCommandA・B」の初期設定(「ConcreteReceiver」を命令の受取手としてセット)、
    // それらの命令を格納 した「Invoker」の起動を行います。 「Command」パターンを適用したクラスを利用し処理します。

    let mut r = Robot::new();
    let mut invoker = Invoker::new(&mut r);
    
    assert_eq!(*invoker.target(),
               Robot {
                   x: 0,
                   y: 0,
                   dx: 0,
                   dy: 1,
               });

    {
        // 
        use RoboCommand::*;
        invoker.append_command(TurnRight);
        invoker.append_command(TurnLeft);
        invoker.append_command(MoveForward);
    }

    invoker.execute_all_commands();
    assert_eq!(*invoker.target(),
               Robot {
                   x: 0,
                   y: 1,
                   dx: 0,
                   dy: 1,
               });

    invoker.undo();
    assert_eq!(*invoker.target(),
               Robot {
                   x: 0,
                   y: 0,
                   dx: 0,
                   dy: 1,
               });

    invoker.undo();
    assert_eq!(*invoker.target(),
               Robot {
                   x: 0,
                   y: 0,
                   dx: 1,
                   dy: 0,
               });
}