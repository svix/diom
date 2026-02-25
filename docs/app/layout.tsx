import { Footer, Layout, Navbar } from 'nextra-theme-docs'
import { Head } from 'nextra/components'
import { getPageMap } from 'nextra/page-map'
import 'nextra-theme-docs/style.css'

export const metadata = {
  title: {
    default: 'Nextra – Next.js Static Site Generator',
    template: '%s | Nextra'
  },
  openGraph: {
    url: 'https://nextra.site',
    siteName: 'Nextra',
    locale: 'en_US',
    type: 'website'
  },
  // For more information on metadata API, see: https://nextjs.org/docs/app/building-your-application/optimizing/metadata
  icons: {
    icon: '/favicon.svg'
  }
}

const navbar = (
  <Navbar
    logo={<b>Coyote</b>}
    // ... Your additional navbar options
  />
)
const footer = <Footer>Copyright © Coyote.</Footer>

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html
      // Not required, but good for SEO
      lang="en"
      // Required to be set
      dir="ltr"
      // Suggested by `next-themes` package https://github.com/pacocoursey/next-themes#with-app
      suppressHydrationWarning
    >
      <Head
      // ... Your additional head options
      >
        {/* Your additional tags should be passed as `children` of `<Head>` element */}
      </Head>
      <body>
        <Layout
          // banner={banner}
          navbar={navbar}
          pageMap={await getPageMap()}
          docsRepositoryBase="https://github.com/shuding/nextra/tree/main/docs"
          editLink={null}
          feedback={{content: null}}
          footer={footer}
        >
          {children}
        </Layout>
      </body>
    </html>
  )
}
