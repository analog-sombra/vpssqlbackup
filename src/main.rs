use std::process::Command;

use cron_job::CronJob;

#[tokio::main]
async fn main() {
    // Create CronJob
    let mut cron = CronJob::default();
    // Add the function
    // cron.new_job("* * * * * *", run_on_cron);
    cron.new_job("45 16 * * *", run_on_cron);
    // Start job
    let _ = cron.start();
}

fn run_on_cron() {
    let mut command = Command::new("bash");
    command.arg("/root/script/full_bkp.sh");
}
