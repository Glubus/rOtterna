import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Select } from "rsc-daisyui";
import { usePacks } from "./hooks/usePacks";
import { PackCard } from "./components/PackCard";
import { Settings } from "./components/Settings";
import "./App.css";

interface SortOption {
  value: string;
  label: string;
}

function App() {
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [sortField, setSortField] = useState("name");
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("asc");
  const [sortOptions, setSortOptions] = useState<SortOption[]>([]);
  const [showSettings, setShowSettings] = useState(false);
  const [downloading, setDownloading] = useState<Set<number>>(new Set());
  const [downloaded, setDownloaded] = useState<Set<number>>(new Set());
  
  // Build sort string: "-field" for descending, "field" for ascending
  const sortString = sortOrder === "desc" ? `-${sortField}` : sortField;
  const { packs, loading, error } = usePacks({ page, limit: 12, sort: sortString, search });
  
  // Load sort options on startup
  useEffect(() => {
    const loadSortOptions = async () => {
      try {
        const options = await invoke<SortOption[]>("get_sort_options");
        setSortOptions(options);
      } catch (err) {
        console.error("[App] Error loading sort options:", err);
      }
    };
    loadSortOptions();
  }, []);
  
  // Clean up downloads state when packs change (e.g., page change)
  useEffect(() => {
    // Keep only downloads for packs that are still in the current list
    setDownloading((prev) => {
      const packIds = new Set(packs.map((p) => p.id));
      return new Set([...prev].filter((id) => packIds.has(id)));
    });
  }, [packs]);

  if (showSettings) {
    return <Settings onBack={() => setShowSettings(false)} />;
  }

  return (
    <main className="container mx-auto p-4">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-4xl font-bold">Etterna Packs</h1>
        <button
          className="btn btn-outline btn-sm"
          onClick={() => setShowSettings(true)}
        >
          Settings
        </button>
      </div>

      <div className="mb-6 flex gap-2 items-center flex-wrap">
        <input
          type="text"
          placeholder="Search for a pack..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="input input-bordered flex-1 min-w-[200px]"
        />
        <button className="btn btn-primary" onClick={() => setPage(1)}>
          Search
        </button>
        
        <Select
          value={sortField}
          onChange={(e) => {
            if (e.target.value) {
              setSortField(e.target.value);
              setPage(1);
            }
          }}
          className="select-bordered"
        >
          <option value="" disabled>Sort by</option>
          {sortOptions.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </Select>
        
        <Select
          value={sortOrder}
          onChange={(e) => {
            setSortOrder(e.target.value as "asc" | "desc");
            setPage(1);
          }}
          className="select-bordered"
        >
          <option value="asc">Ascending</option>
          <option value="desc">Descending</option>
        </Select>
      </div>

      {loading && (
        <div className="flex justify-center">
          <span className="loading loading-spinner loading-lg"></span>
        </div>
      )}
      {error && (
        <div className="alert alert-error">
          <span>Error: {error}</span>
      </div>
      )}

      {!loading && !error && (
        <div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-6">
            {packs.map((pack) => (
              <PackCard
                key={pack.id}
                pack={pack}
                isDownloading={downloading.has(pack.id)}
                isDownloaded={downloaded.has(pack.id)}
                onDownloadStart={() => {
                  setDownloading((prev) => new Set(prev).add(pack.id));
                }}
                onDownloadComplete={() => {
                  setDownloading((prev) => {
                    const newSet = new Set(prev);
                    newSet.delete(pack.id);
                    return newSet;
                  });
                  setDownloaded((prev) => new Set(prev).add(pack.id));
                }}
                onDownloadError={() => {
                  setDownloading((prev) => {
                    const newSet = new Set(prev);
                    newSet.delete(pack.id);
                    return newSet;
                  });
                }}
              />
            ))}
          </div>

          <div className="flex justify-center items-center gap-4">
            <button
              className="btn btn-outline"
              onClick={() => setPage((p) => Math.max(1, p - 1))}
              disabled={page === 1}
            >
              Previous
            </button>
            <span>Page {page}</span>
            <button className="btn btn-outline" onClick={() => setPage((p) => p + 1)}>
              Next
            </button>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;
