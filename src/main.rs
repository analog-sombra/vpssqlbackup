use std::{process::Command, time::Duration};
use job_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    // Create CronJob
    // let mut cron = CronJob::default();
    // // Add the function
    // cron.new_job("* * * * * *", run_each_second);
    // cron.new_job("45 16 * * *", run_on_cron);
    // // Start job
    // let _ = cron.start();

    // loop {
    //     std::thread::sleep(Duration::from_millis(500));
    // }

    let mut sched = JobScheduler::new();

    sched.add(Job::new("* 55 16 * * *".parse().unwrap(), || {
        let mut command = Command::new("bash");
        command.arg("/root/script/full_bkp.sh");
    }));

    sched.add(Job::new("* * * * * *".parse().unwrap(), || {
        println!("i run every second");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
