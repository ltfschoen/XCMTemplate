import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import { SmartAccountProvider  } from './contexts/SmartAccountAcontext'
import { Web3AuthProvider } from './contexts/SocialLoginContext'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    {/* <Web3AuthProvider>
      <SmartAccountProvider> */}
        <App />
      {/* </SmartAccountProvider>
    </Web3AuthProvider> */}
  </React.StrictMode>,
)
