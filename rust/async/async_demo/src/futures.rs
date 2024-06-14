use std::task;


pub trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> task::Poll<Self::Output>;
}

// Future 需要被执行器 poll(轮询)后才能运行
// 若在当前 poll 中，Future 可以被完成，返回 Poll::Ready(result)
// 否则返回 Poll::Pending，并且安排一个 wake 函数，在未来 Future 准备好进一步执行时调用
// enum Poll<T> {
//     Ready(T),
//     Pending,
// }


// pub struct SocketRead<'a> {
//     socket: &'a Socket,
// }
//
// impl SimpleFuture for SocketRead<'_> {
//     type Output = Vec<u8>;
//
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         if self.socket.has_data_to_read() {
//             // socket有数据，写入buffer中并返回
//             Poll::Ready(self.socket.read_buf())
//         } else {
//             // socket中还没数据
//             //
//             // 注册一个`wake`函数，当数据可用时，该函数会被调用，
//             // 然后当前Future的执行器会再次调用`poll`方法，此时就可以读取到数据
//             self.socket.set_readable_callback(wake);
//             Poll::Pending
//         }
//     }
// }
//
