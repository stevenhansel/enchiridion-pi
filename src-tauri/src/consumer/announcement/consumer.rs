use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::{
    domain::Device, events::ApplicationEvent, queue::Consumer, services::AnnouncementService,
};

use super::definition::{AnnouncementConsumerPayload, AnnouncementSyncAction};

const QUEUE_NAME_PREFIX: &'static str = "device-queue-";

pub async fn consume(
    device: Device,
    handle: AppHandle,
    redis: deadpool_redis::Pool,
    announcement_service: Arc<AnnouncementService>,
) {
    let queue_name = format!("{}{}", QUEUE_NAME_PREFIX, device.device_id);
    println!("queue_name: {}", queue_name);
    let mut consumer = Consumer::new(redis, queue_name);

    loop {
        let pending_message_id = match consumer.get_pending_message_id().await {
            Ok(id) => id,
            Err(e) => {
                println!("An error occurred while getting pending message: {}", e);
                continue;
            }
        };

        if let Some(message_id) = pending_message_id {
            handle_pending_messages(
                &handle,
                &mut consumer,
                announcement_service.clone(),
                message_id,
            )
            .await;
        } else {
            handle_upcoming_messages(&handle, &mut consumer, announcement_service.clone()).await;
        }

        handle
            .emit_all(ApplicationEvent::MediaUpdateEnd.tag(), "emitted")
            .expect("Error when emitting");
    }
}

async fn handle_pending_messages(
    handle: &AppHandle,
    consumer: &mut Consumer,
    announcement_service: Arc<AnnouncementService>,
    message_id: String,
) {
    let data = match consumer
        .read_by_message_id::<AnnouncementConsumerPayload>(message_id.to_string())
        .await
    {
        Ok(res) => res,
        Err(e) => {
            println!("An error occurred while consuming data: {}", e);
            return;
        }
    };

    if data.len() == 0 {
        if let Err(e) = consumer.ack(message_id.to_string()).await {
            println!(
                "An error occurred while acknowledging the announcement: {}",
                e
            );
        }

        return;
    }

    handle
        .emit_all(ApplicationEvent::MediaUpdateStart.tag(), "emitted")
        .expect("Error when emitting");

    let (message_id, payload) = &data[0];
    println!(
        "Start processing announcement with message_id: {}",
        message_id.to_string()
    );

    if let Err(e) = process_announcement(announcement_service, payload).await {
        println!(
            "Something when wrong when processing the announcements: {}",
            e
        );
        return;
    }

    if let Err(e) = consumer.ack(message_id.to_string()).await {
        println!(
            "An error occurred while acknowledging the announcement: {}",
            e
        );
        return;
    }

    println!(
        "Finished processing announcement with message_id: {}",
        message_id.to_string()
    );
}

async fn handle_upcoming_messages(
    handle: &AppHandle,
    consumer: &mut Consumer,
    announcement_service: Arc<AnnouncementService>,
) {
    let data = match consumer.consume::<AnnouncementConsumerPayload>().await {
        Ok(res) => res,
        Err(e) => {
            println!("An error occurred while consuming data: {}", e);
            return;
        }
    };
    if data.len() == 0 {
        return;
    }

    handle
        .emit_all(ApplicationEvent::MediaUpdateStart.tag(), "emitted")
        .expect("Error when emitting");

    let (message_id, payload) = &data[0];
    println!(
        "Start processing announcement with message_id: {}",
        message_id.to_string()
    );

    if let Err(e) = process_announcement(announcement_service, payload).await {
        println!(
            "Something when wrong when processing the announcements: {}",
            e
        );
        return;
    }

    if let Err(e) = consumer.ack(message_id.to_string()).await {
        println!(
            "An error occurred while acknowledging the announcement: {}",
            e
        );
        return;
    }

    println!(
        "Finished processing announcement with message_id: {}",
        message_id.to_string()
    );
}

async fn process_announcement(
    announcement_service: Arc<AnnouncementService>,
    payload: &AnnouncementConsumerPayload,
) -> Result<(), String> {
    match payload.action {
        AnnouncementSyncAction::Create => {
            if let Err(e) = announcement_service.create(payload).await {
                return Err(e.to_string());
            };
        }
        AnnouncementSyncAction::Delete => {
            if let Err(e) = announcement_service.delete(payload).await {
                return Err(e.to_string());
            }
        }
        AnnouncementSyncAction::Resync => {
            if let Err(e) = announcement_service.resync(payload).await {
                return Err(e.to_string());
            }
        }
    };

    Ok(())
}
