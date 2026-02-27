import type { NextConfig } from 'next'
import nextra from 'nextra'

// Set up Nextra with its configuration
const withNextra = nextra({
  // ... Add Nextra-specific options here
})

const ciExportConfig: NextConfig = {
  output: "export",
  images: {
    unoptimized: true,
  },
};

const nextConfig: NextConfig = {
  devIndicators: false,
  ...(process.env.CI && ciExportConfig),
}

export default withNextra(nextConfig)
