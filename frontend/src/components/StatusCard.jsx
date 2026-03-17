function StatusCard({ appName, version, name, vpsAddr }) {
  if (!appName || !version || !name) {
    return null
  }

  const vpsDisplay = vpsAddr ? `${vpsAddr[0]}:${vpsAddr[1]}` : '未配置'

  return (
    <div className="bg-white rounded-lg shadow p-6 mb-6">
      <h2 className="text-xl font-semibold mb-4 text-gray-700">系统状态</h2>
      <div className="space-y-2">
        <p className="text-gray-600">应用名称: <span className="font-medium text-gray-800">{appName}</span></p>
        <p className="text-gray-600">版本: <span className="font-medium text-gray-800">{version}</span></p>
        <p className="text-gray-600">主机名: <span className="font-medium text-gray-800">{name}</span></p>
        <p className="text-gray-600">VPS地址: <span className="font-medium text-gray-800">{vpsDisplay}</span></p>
      </div>
    </div>
  )
}

export default StatusCard
