import { useState } from 'react'
import axios from 'axios'

function DynIPList({ dynIPs, onRefresh }) {
  const [newName, setNewName] = useState('')
  const [newIP, setNewIP] = useState('')
  const [loading, setLoading] = useState(false)

  const handleSubmit = async (e) => {
    e.preventDefault()
    if (!newName || !newIP) return

    setLoading(true)
    try {
      await axios.post('/api/set_dynip', {
        name: newName,
        ip: newIP
      })
      setNewName('')
      setNewIP('')
      onRefresh()
    } catch (err) {
      console.error('Failed to add dynip:', err)
      alert('添加失败')
    } finally {
      setLoading(false)
    }
  }

  if (!dynIPs) {
    return null
  }

  const entries = Object.entries(dynIPs || {})

  return (
    <div className="bg-white rounded-lg shadow p-6 mb-6">
      <h2 className="text-xl font-semibold mb-4 text-gray-700">动态IP映射</h2>

      <form onSubmit={handleSubmit} className="mb-4 p-4 bg-gray-50 rounded space-y-2">
        <div className="grid grid-cols-2 gap-2">
          <input
            type="text"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            placeholder="设备名称"
            className="px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <input
            type="text"
            value={newIP}
            onChange={(e) => setNewIP(e.target.value)}
            placeholder="IP地址"
            className="px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <button
          type="submit"
          disabled={loading}
          className="w-full bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 disabled:bg-blue-300 transition-colors"
        >
          {loading ? '添加中...' : '添加IP映射'}
        </button>
      </form>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-4 py-2 text-left text-gray-600">设备名称</th>
              <th className="px-4 py-2 text-left text-gray-600">IP地址</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {entries.length > 0 ? (
              entries.map(([name, ip], index) => (
                <tr key={index}>
                  <td className="px-4 py-2 text-gray-800">{name}</td>
                  <td className="px-4 py-2 text-gray-600">{ip || 'N/A'}</td>
                </tr>
              ))
            ) : (
              <tr>
                <td colSpan={2} className="px-4 py-2 text-gray-600 text-center">
                  无动态IP映射
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  )
}

export default DynIPList
