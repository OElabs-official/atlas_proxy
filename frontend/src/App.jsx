import { useState, useEffect } from 'react'
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify'
import 'react-toastify/dist/ReactToastify.css'
import Layout from './components/Layout'
import StatusCard from './components/StatusCard'
import VpsConfig from './components/VpsConfig'
import StaticPorts from './components/StaticPorts'
import DynIPList from './components/DynIPList'
import LocalIP from './components/LocalIP'

function App() {
  const [status, setStatus] = useState(null)
  const [config, setConfig] = useState(null)
  const [staticPorts, setStaticPorts] = useState([])
  const [dynIPs, setDynIPs] = useState({})
  const [localIP, setLocalIP] = useState([])
  const [loading, setLoading] = useState(true)

  const fetchAllData = async () => {
    try {
      const [statusRes, cfgRes, portsRes, dynRes, localRes] = await Promise.all([
        axios.get('/api/'),
        axios.get('/api/cfg'),
        axios.get('/api/port'),
        axios.get('/api/list_dynip'),
        axios.get('/api/localip')
      ])

      setStatus(statusRes.data)
      setConfig(cfgRes.data)
      setStaticPorts(cfgRes.data.static_port_forwards || [])
      setDynIPs(dynRes.data || {})
      setLocalIP(localRes.data || [])
    } catch (err) {
      console.error('Failed to fetch data:', err)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchAllData()
  }, [])

  const showToast = (message, type = 'success') => {
    if (type === 'success') {
      toast.success(message)
    } else {
      toast.error(message)
    }
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600">加载中...</p>
        </div>
      </div>
    )
  }

  return (
    <>
      <ToastContainer position="top-right" autoClose={3000} hideProgressBar />
      <Layout>
        <StatusCard
          appName={status?.[0] || config?.app_name}
          version={status?.[1] || config?.version}
          name={config?.name}
          vpsAddr={config?.registration}
        />

        <VpsConfig config={config} onUpdate={fetchAllData} />

        <StaticPorts ports={staticPorts} />

        <DynIPList dynIPs={dynIPs} onRefresh={fetchAllData} />

        <LocalIP ip={localIP} />
      </Layout>
    </>
  )
}

export default App
