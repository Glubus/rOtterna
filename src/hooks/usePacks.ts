import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface Pack {
  id: number;
  name: string;
  play_count: number;
  song_count: number;
  banner_path: string;
  bannerTinyThumb: string;
  bannerSrcSet: string;
  contains_nsfw: boolean;
  size: string;
  overall: number;
  stream: number;
  jumpstream: number;
  handstream: number;
  jacks: number;
  chordjacks: number;
  stamina: number;
  technical: number;
  tags: Array<{ type: string; name: string }>;
  download: string;
  magnet: string;
}

export interface PacksResponse {
  data: Pack[];
  links: {
    first: string;
    last: string;
    prev: string | null;
    next: string | null;
  };
  meta: {
    current_page: number;
    from: number;
    last_page: number;
    links: Array<{
      url: string | null;
      label: string;
      active: boolean;
    }>;
    path: string;
    per_page: number;
    to: number;
    total: number;
  };
}

interface FetchPacksParams {
  page?: number;
  limit?: number;
  sort?: string;
  search?: string;
}

export function usePacks(params: FetchPacksParams = {}) {
  const [packs, setPacks] = useState<Pack[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [meta, setMeta] = useState<PacksResponse["meta"] | null>(null);
  
  // Use ref to track previous params and avoid duplicate calls
  const prevParamsRef = useRef<string>("");
  const paramsKey = JSON.stringify({ 
    page: params.page, 
    limit: params.limit, 
    sort: params.sort, 
    search: params.search 
  });

  useEffect(() => {
    // Skip if params haven't actually changed
    if (prevParamsRef.current === paramsKey) {
      return;
    }
    prevParamsRef.current = paramsKey;

    const fetchPacks = async () => {
      try {
        console.log("[usePacks] Fetching packs with params:", params);
        setLoading(true);
        setError(null);
        const response = await invoke<PacksResponse>("fetch_packs", {
          page: params.page,
          limit: params.limit,
          sort: params.sort,
          search: params.search,
        });
        console.log("[usePacks] Received response:", response);
        console.log("[usePacks] Number of packs:", response.data.length);
        setPacks(response.data);
        setMeta(response.meta);
      } catch (err) {
        console.error("[usePacks] Error fetching packs:", err);
        setError(err instanceof Error ? err.message : "Error loading packs");
      } finally {
        setLoading(false);
      }
    };

    fetchPacks();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [paramsKey]);

  return { packs, loading, error, meta };
}

