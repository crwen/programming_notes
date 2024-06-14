use std::pin::Pin;

use futures::{io, Stream};

// 每次只处理一个元素
//
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt;
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt;
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}

// 并发处理
//
pub async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;
    stream
        .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move { Ok(()) })
        .await?;
    Ok(())
}
