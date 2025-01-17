import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Avoid from rendering twice in development mode
  reactStrictMode: false,
};

export default nextConfig;
