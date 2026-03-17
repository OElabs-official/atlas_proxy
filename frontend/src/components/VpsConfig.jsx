import { useState } from 'react'

function VpsConfig({ config, onUpdate }) {
  const [vpsAddr, setVpsAddr] = useState(config?.registration?.[0] || '127.0.0.1')
  const [vpsPort, setVpsPort] = useState(config?.registration?.[1] || 1025)
  const [loading, setLoading] = useState(false)

  const handleSubmit = async (e) => {
    e.preventDefault()
    setLoading(true)

    try {
      await axios.post('/api/set_vps', {
        vps_addr: vpsAddr,
        vps_port: parseInt(vpsPort)
      })
      onUpdate()
    } catch (err) {
      console.error('Failed to update VPS config:', err)
      alert('更新失败')
    } finally {
      setLoading(false)
    }
  }

  if (!config) {
    return null
  }

  const currentVps = config.registration
    ? `${config.registration[0]}:${config.registration[1]}`
    : '未配置'

  return (
    <div className="bg-white rounded-lg shadow p-6 mb-6">
      <h2 className="text-xl font-semibold mb-4 text-gray-700">VPS配置</h2>
      <div className="mb-4 p-4 bg-gray-50 rounded">
        <p className="text-gray-600">当前VPS地址: <span className="font-medium text-gray-800">{currentVps}</span></p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">VPS地址</label>
            <input
              type="text"
              value={vpsAddr}
              onChange={(e) => setVpsAddr(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="例如: 127.0.0.1"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">VPS端口</label>
            <input
              type="number"
              value={vpsPort}
              onChange={(e) => setVpsPort(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="例如: 1025"
            />
          </div>
        </div>
        <button
          type="submit"
          disabled={loading}
          className="w-full bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 disabled:bg-blue-300 transition-colors"
        >
          {loading ? '保存中...' : '保存VPS配置'}
        </button>
      </form>
    </div>
  )
}

export default VpsConfig
