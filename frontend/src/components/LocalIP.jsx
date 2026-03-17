import { useState } from 'react'
import axios from 'axios'

function LocalIP({ ip, onRegister }) {
  const [loading, setLoading] = useState(false)

  const handleRegister = async () => {
    setLoading(true)
    try {
      await axios.get('/api/regist')
      alert('注册成功')
    } catch (err) {
      console.error('Registration failed:', err)
      alert('注册失败')
    } finally {
      setLoading(false)
    }
  }

  if (!ip) {
    return null
  }

  return (
    <div className="bg-white rounded-lg shadow p-6 mb-6">
      <h2 className="text-xl font-semibold mb-4 text-gray-700">本机IP地址</h2>
      <div className="mb-4 p-4 bg-gray-50 rounded">
        <p className="text-gray-600 mb-2">当前IP地址:</p>
        <ul className="list-disc list-inside text-gray-800">
          {ip.map((addr, index) => (
            <li key={index}>{addr}</li>
          ))}
        </ul>
      </div>
      <button
        onClick={handleRegister}
        disabled={loading}
        className="w-full bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600 disabled:bg-green-300 transition-colors"
      >
        {loading ? '注册中...' : '注册到VPS'}
      </button>
    </div>
  )
}

export default LocalIP
