use anyhow::Result;
use watchexec::Watchexec;
use watchexec_signals::Signal;

#[tokio::main]
async fn main() -> Result<()> {
    let wx = Watchexec::default();
    wx.config.pathset(vec!["./tmp"]);
    wx.config.on_action(move |mut action| {
        println!("caught an action");
        // This looses the ability to use Ctrl+c
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit();
        }

        // This works
        // if action
        //     .signals()
        //     .any(|sig| sig == Signal::Interrupt || sig == Signal::Terminate)
        // {
        //     action.quit();
        // }

        action
    });
    println!("Starting watcher");
    let _ = wx.main().await?;
    println!("Process complete.");
    Ok(())
}
