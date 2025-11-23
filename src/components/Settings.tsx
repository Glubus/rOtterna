"use client";

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface Settings {
  hp_drain_rate: number;
  overall_difficulty: number;
  song_path: string;
}

interface SettingsProps {
  onBack?: () => void;
}

export function Settings({ onBack }: SettingsProps = {}) {
  const [settings, setSettings] = useState<Settings>({
    hp_drain_rate: 8.0,
    overall_difficulty: 9.0,
    song_path: "",
  });
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const loaded = await invoke<Settings>("get_settings");
      setSettings(loaded);
    } catch (err) {
      console.error("[Settings] Error loading settings:", err);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      setSaving(true);
      await invoke("set_settings", { settings });
      alert("Settings saved successfully!");
    } catch (err) {
      console.error("[Settings] Error saving settings:", err);
      alert(`Error saving settings: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setSaving(false);
    }
  };

  const handleSelectSongPath = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Song Directory",
      });
      if (selected && typeof selected === "string") {
        setSettings((prev) => ({ ...prev, song_path: selected }));
      }
    } catch (err) {
      console.error("[Settings] Error selecting song path:", err);
    }
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center h-screen">
        <span className="loading loading-spinner loading-lg"></span>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-4 max-w-2xl">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-4xl font-bold">Settings</h1>
        {onBack && (
          <button className="btn btn-outline btn-sm" onClick={onBack}>
            Back
          </button>
        )}
      </div>

      <div className="card bg-base-100 shadow-xl">
        <div className="card-body">
          <div className="form-control mb-4">
            <label className="label">
              <span className="label-text font-semibold">HP Drain Rate</span>
            </label>
            <input
              type="number"
              step="0.1"
              min="0"
              max="10"
              className="input input-bordered"
              value={settings.hp_drain_rate}
              onChange={(e) =>
                setSettings((prev) => ({
                  ...prev,
                  hp_drain_rate: parseFloat(e.target.value) || 0,
                }))
              }
            />
          </div>

          <div className="form-control mb-4">
            <label className="label">
              <span className="label-text font-semibold">Overall Difficulty</span>
            </label>
            <input
              type="number"
              step="0.1"
              min="0"
              max="10"
              className="input input-bordered"
              value={settings.overall_difficulty}
              onChange={(e) =>
                setSettings((prev) => ({
                  ...prev,
                  overall_difficulty: parseFloat(e.target.value) || 0,
                }))
              }
            />
          </div>

          <div className="form-control mb-6">
            <label className="label">
              <span className="label-text font-semibold">Song Path</span>
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                className="input input-bordered flex-1"
                placeholder="Path where song folders will be copied (e.g., C:/osu!/Songs)"
                value={settings.song_path}
                onChange={(e) =>
                  setSettings((prev) => ({
                    ...prev,
                    song_path: e.target.value,
                  }))
                }
              />
              <button
                className="btn btn-outline"
                onClick={handleSelectSongPath}
              >
                Browse
              </button>
            </div>
            <label className="label">
              <span className="label-text-alt">
                Leave empty to keep songs in the download folder
              </span>
            </label>
          </div>

          <div className="card-actions justify-end">
            <button
              className="btn btn-primary"
              onClick={handleSave}
              disabled={saving}
            >
              {saving ? (
                <>
                  <span className="loading loading-spinner loading-sm"></span>
                  Saving...
                </>
              ) : (
                "Save Settings"
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

