use tokio::net::{TcpListener, TcpStream};

#[tokio_uring::test]
async fn use_tokio_types_from_runtime() {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let task = tokio::spawn(async move {
        let _socket = TcpStream::connect(addr).await.unwrap();
    });

    // Accept a connection
    let (_socket, _) = listener.accept().await.unwrap();

    // Wait for the task to complete
    task.await.unwrap();
}

#[tokio_uring::test]
async fn spawn_a_task() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let cell = Rc::new(RefCell::new(1));
    let c = cell.clone();
    let handle = tokio_uring::spawn(async move {
        *c.borrow_mut() = 2;
    });

    handle.await.unwrap();
    assert_eq!(2, *cell.borrow());
}
