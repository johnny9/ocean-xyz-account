use ocean_xyz_account::get_earnings;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let account_name = "bc1qqdz7tp7tgtpqrscw2p02nah4lwf4sa4v75wdal";
    let earnings = get_earnings(account_name).await?;

    for entry in earnings.iter().take(5) {
        println!("{:?}", entry);
    }

    Ok(())
}