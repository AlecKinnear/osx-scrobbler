//! Handles background metadata enrichment for tracks.
//!
//! This module is responsible for taking a basic track, fetching additional
//! metadata from external services (like Last.fm for album art), and then
//! sending the enriched track back to the main thread for UI updates.
//!
//! All operations here are asynchronous and designed to run in a background
//! task to avoid blocking the main UI thread.

use crate::scrobbler::Track;
use anyhow::Result;
use tokio::sync::mpsc::Sender;

/// A message sent from the enrichment task back to the main thread.
#[derive(Debug)]
pub enum EnrichmentUpdate {
    TrackUpdated(Track),
}

/// Represents the metadata enrichment engine.
pub struct EnrichmentEngine {
    // In the future, this could hold API clients for MusicBrainz, etc.
    // For now, we just need the sender to communicate back to the UI.
    ui_sender: Sender<EnrichmentUpdate>,
}

impl EnrichmentEngine {
    /// Creates a new enrichment engine.
    pub fn new(ui_sender: Sender<EnrichmentUpdate>) -> Self {
        Self { ui_sender }
    }

    /// Spawns a background task to enrich a track.
    ///
    /// This function returns immediately, and the enrichment happens on a
    /// separate tokio task.
    pub fn enrich_track(&self, track: Track) {
        let sender = self.ui_sender.clone();

        tokio::spawn(async move {
            log::debug!("Starting enrichment for track: {}", track.artist);
            let mut enriched_track = track;

            // --- Placeholder for actual enrichment logic ---
            // This is where you would make async HTTP calls to Last.fm,
            // MusicBrainz, or other services.
            // For example:
            // if let Ok(art_url) = lastfm::get_album_art(&enriched_track).await {
            //     enriched_track.album_art_url = Some(art_url);
            // }

            // Send the updated track back to the main thread.
            if let Err(e) = sender.send(EnrichmentUpdate::TrackUpdated(enriched_track)).await {
                log::error!("Failed to send enriched track back to UI thread: {}", e);
            }
        });
    }
}