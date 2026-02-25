import { Footer, Layout, Navbar } from 'nextra-theme-docs'
import { Head } from 'nextra/components'
import { getPageMap } from 'nextra/page-map'
import 'nextra-theme-docs/style.css'

const siteData = {
  name: "Diom",
  url: 'https://www.svix.com/diom',
}

export const metadata = {
  title: {
    default: `${siteData.name} – the backend survival kit`,
    template: `%s | ${siteData.name}`
  },
  // TODO: verify and improve opengraph.
  openGraph: {
    url: siteData.url,
    siteName: siteData.name,
    locale: 'en_US',
    type: 'article',
  },
  // For more information on metadata API, see: https://nextjs.org/docs/app/building-your-application/optimizing/metadata
  icons: {
    icon: '/favicon.svg'
  }
}

const navbar = (
  <Navbar
    logo={<b>Diom</b>}
  />
)
const footer = <Footer>Copyright © Diom.</Footer>

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html
      lang="en"
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
