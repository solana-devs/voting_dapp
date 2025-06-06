'use client'

import './globals.css'
import AppWalletProvider from './components/AppWalletProvider'
import { ReactQueryProvider } from './react-query-provider'
import Header from './components/Header'
import 'react-toastify/dist/ReactToastify.css'
import { ToastContainer } from 'react-toastify'
import { Provider } from 'react-redux'
import { store } from './store'

const metadata = {
  title: 'voting',
  description: 'Generated by create-solana-dapp',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <ReactQueryProvider>
          <Provider store={store}>
            <AppWalletProvider>
              <Header />
              <main className="max-w-6xl mx-auto">{children}</main>
              <ToastContainer
                position="bottom-center"
                autoClose={5000}
                hideProgressBar={false}
                newestOnTop={false}
                closeOnClick
                rtl={false}
                pauseOnFocusLoss
                draggable
                pauseOnHover
                theme="dark"
              />
            </AppWalletProvider>
          </Provider>
        </ReactQueryProvider>
      </body>
    </html>
  )
}
