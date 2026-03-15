import { useState, useEffect } from 'react'
import axios from 'axios'

function App() {
  const [vpsStatus, setVpsStatus] = useState(null)
  const [hosts, setHosts] = useState([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetchStatus()
    fetchHosts()
  }, [])

  const fetchStatus = async () => {
    try {
      const res = await axios.get('/api/v1/status')
      setVpsStatus(res.data)
    } catch (err) {
      console.error('Failed to fetch status:', err)
    }
  }

  const fetchHosts = async () => {
    try {
      const res = await axios.get('/api/v1/hosts')
      setHosts(res.data)
    } catch (err) {
      console.error('Failed to fetch hosts:', err)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <nav className="bg-white shadow">
        <div className="container mx-auto px-4 py-4">
          <h1 className="text-2xl font-bold text-gray-800">Atlas Proxy</h1>
        </div>
      </nav>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        {/* Status Card */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-xl font-semibold mb-4 text-gray-700">系统状态</h2>
            {loading ? (
              <p className="text-gray-600">加载中...</p>
            ) : vpsStatus ? (
              <div className="space-y-2">
                <p className="text-gray-600">状态: <span className="text-green-600 font-semibold">运行中</span></p>
                <p className="text-gray-600">模式: {vpsStatus.mode}</p>
                <p className="text-gray-600">时间: {vpsStatus.timestamp}</p>
              </div>
            ) : (
              <p className="text-gray-600">无法连接到后端服务</p>
            )}
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-xl font-semibold mb-4 text-gray-700">快速操作</h2>
            <div className="space-y-2">
              <button className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 w-full">
                注册 IP 地址
              </button>
              <button className="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600 w-full">
                添加端口转发
              </button>
            </div>
          </div>
        </div>

        {/* Port Forwards Table */}
        <div className="bg-white rounded-lg shadow p-6 mt-6">
          <h2 className="text-xl font-semibold mb-4 text-gray-700">端口转发规则</h2>
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-2 text-left text-gray-600">名称</th>
                <th className="px-4 py-2 text-left text-gray-600">监听端口</th>
                <th className="px-4 py-2 text-left text-gray-600">转发目标</th>
                <th className="px-4 py-2 text-left text-gray-600">状态</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200">
              {hosts.length > 0 ? (
                hosts.map((host, index) => (
                  <tr key={index} className="hover:bg-gray-50">
                    <td className="px-4 py-2 text-gray-800">{host.name}</td>
                    <td className="px-4 py-2 text-gray-600">{host.port}</td>
                    <td className="px-4 py-2 text-gray-600">{host.ip_v4 || host.ip_v6 || 'N/A'}</td>
                    <td className="px-4 py-2">
                      <span className="text-green-600">在线</span>
                    </td>
                  </tr>
                ))
              ) : (
                <tr>
                  <td colSpan={4} className="px-4 py-2 text-gray-600 text-center">
                    无规则
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white shadow mt-auto">
        <div className="container mx-auto px-4 py-4 text-center text-gray-600">
          <p>Atlas Proxy v0.1.0 - 分布式端口转发代理</p>
        </div>
      </footer>
    </div>
  )
}

export default App
