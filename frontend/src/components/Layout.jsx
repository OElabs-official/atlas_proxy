import { useState, useEffect } from 'react'
import axios from 'axios'

function Layout({ children }) {
  return (
    <div className="min-h-screen bg-gray-100 flex flex-col">
      <nav className="bg-white shadow">
        <div className="container mx-auto px-4 py-4">
          <h1 className="text-2xl font-bold text-gray-800">Atlas Proxy</h1>
        </div>
      </nav>

      <main className="flex-grow container mx-auto px-4 py-8">
        {children}
      </main>

      <footer className="bg-white shadow">
        <div className="container mx-auto px-4 py-4 text-center text-gray-600">
          <p>Atlas Proxy v0.1.0 - 分布式端口转发代理</p>
        </div>
      </footer>
    </div>
  )
}

export default Layout
