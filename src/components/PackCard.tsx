"use client";

import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Pack } from "../hooks/usePacks";

interface PackCardProps {
  pack: Pack;
  isDownloading: boolean;
  isDownloaded: boolean;
  onDownloadStart: () => void;
  onDownloadComplete: () => void;
  onDownloadError: () => void;
}

interface DownloadProgress {
  packId: number;
  downloaded: number;
  total: number;
  stage: "downloading" | "extracting" | "converting";
}

export function PackCard({
  pack,
  isDownloading: externalIsDownloading,
  isDownloaded,
  onDownloadStart,
  onDownloadComplete,
  onDownloadError,
}: PackCardProps) {
  const [progress, setProgress] = useState<DownloadProgress | null>(null);
  const unlistenRef = useRef<(() => void) | null>(null);

  // Re-initialize listener if download is in progress when component mounts
  useEffect(() => {
    if (externalIsDownloading && !unlistenRef.current) {
      const setupListener = async () => {
        try {
          const unlisten = await listen<DownloadProgress>(
            `download-progress-${pack.id}`,
            (event) => {
              setProgress(event.payload);
            }
          );
          unlistenRef.current = unlisten;
        } catch (err) {
          console.error("[PackCard] Error setting up listener:", err);
        }
      };
      setupListener();
    }

    // Clean up listener on unmount
    return () => {
      if (unlistenRef.current) {
        unlistenRef.current();
        unlistenRef.current = null;
      }
    };
  }, [externalIsDownloading, pack.id]);

  const handleDownload = async () => {
    // Prevent multiple downloads
    if (externalIsDownloading || isDownloaded) {
      return;
    }

    try {
      onDownloadStart();
      setProgress({ packId: pack.id, downloaded: 0, total: 0, stage: "downloading" });

      // Listen for progress events
      const unlisten = await listen<DownloadProgress>(
        `download-progress-${pack.id}`,
        (event) => {
          setProgress(event.payload);
        }
      );
      unlistenRef.current = unlisten;

      console.log("[PackCard] Starting download for pack:", pack.id, pack.download);
      const filePath = await invoke<string>("download_pack", {
        downloadUrl: pack.download,
        packId: pack.id,
      });
      console.log("[PackCard] Download completed:", filePath);

      // Clean up listener
      if (unlistenRef.current) {
        unlistenRef.current();
        unlistenRef.current = null;
      }

      setProgress(null);
      onDownloadComplete();
    } catch (err) {
      console.error("[PackCard] Download error:", err);
      setProgress(null);
      if (unlistenRef.current) {
        unlistenRef.current();
        unlistenRef.current = null;
      }
      onDownloadError();
    }
  };

  const progressPercent = progress
    ? progress.total > 0
      ? Math.round((progress.downloaded / progress.total) * 100)
      : 0
    : 0;

  return (
    <div
      className="card border-2 border-primary shadow-lg relative overflow-hidden h-full bg-transparent"
      style={{
        backgroundImage: pack.banner_path
          ? `url(${pack.banner_path})`
          : undefined,
        backgroundSize: "cover",
        backgroundPosition: "center",
      }}
    >
      {/* Overlay for better text readability */}
      <div className="absolute inset-0 bg-black/85"></div>

      <div className="card-body relative z-10 flex flex-col">
        {/* Title with tooltip */}
        <div className="tooltip tooltip-bottom w-full" data-tip={pack.name}>
          <h2 className="card-title text-white text-lg drop-shadow-lg truncate w-full" title={pack.name}>
            {pack.name}
          </h2>
        </div>

        {/* Stats row: songs, size, rating */}
        <div className="flex gap-2 mb-2">
          <div 
            className="tooltip tooltip-bottom"
            data-tip={`${pack.song_count} songs`}
          >
            <div className="badge badge-outline badge-secondary whitespace-nowrap">
              {pack.song_count}
            </div>
          </div>
          <div 
            className="tooltip tooltip-bottom"
            data-tip={pack.size}
          >
            <div className="badge badge-outline badge-accent whitespace-nowrap">
              {pack.size}
            </div>
          </div>
          <div 
            className="tooltip tooltip-bottom"
            data-tip={`${pack.overall.toFixed(2)} overall`}
          >
            <div className="badge badge-outline badge-warning whitespace-nowrap">
              {pack.overall.toFixed(2)}
            </div>
          </div>
        </div>

        {/* Bottom row: tags, plays, and download button */}
        <div className="flex items-center justify-between gap-2 mt-auto">
          <div className="flex flex-wrap gap-1 items-center flex-1">
            {pack.tags.map((tag, idx) => (
              <div 
                key={idx} 
                className="tooltip tooltip-top"
                data-tip={tag.name}
              >
                <div className="badge badge-outline badge-primary badge-sm">
                  {tag.name}
                </div>
              </div>
            ))}
            <div 
              className="tooltip tooltip-top"
              data-tip={`${pack.play_count.toLocaleString()} plays`}
            >
              <div className="badge badge-outline badge-info badge-sm">
                {pack.play_count.toLocaleString()}
              </div>
            </div>
          </div>
          
          {externalIsDownloading && progress ? (
            <div className="flex items-center gap-2">
              <span className="text-white text-xs drop-shadow-lg">
                {progressPercent}%
              </span>
              <progress
                className="progress progress-primary w-20 h-2"
                value={progressPercent}
                max="100"
              ></progress>
            </div>
          ) : isDownloaded ? (
            <button
              className="btn btn-success btn-sm btn-circle"
              disabled
              title="Downloaded"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-5 w-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </button>
          ) : (
            <button
              className="btn btn-primary btn-sm btn-circle"
              onClick={handleDownload}
              disabled={externalIsDownloading}
              title="Download"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-5 w-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                />
              </svg>
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

