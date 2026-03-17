function StaticPorts({ ports }) {
  if (!ports || ports.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow p-6 mb-6">
        <h2 className="text-xl font-semibold mb-4 text-gray-700">静态端口转发</h2>
        <p className="text-gray-600 text-center">暂无静态端口转发规则</p>
      </div>
    )
  }

  return (
    <div className="bg-white rounded-lg shadow p-6 mb-6">
      <h2 className="text-xl font-semibold mb-4 text-gray-700">静态端口转发</h2>
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-4 py-2 text-left text-gray-600">监听端口</th>
              <th className="px-4 py-2 text-left text-gray-600">转发目标</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {ports.map((port, index) => (
              <tr key={index}>
                <td className="px-4 py-2 text-gray-800">
                  0.0.0.0:{port.output}
                </td>
                <td className="px-4 py-2 text-gray-600">
                  {port.input[0]}:{port.input[1]}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}

export default StaticPorts
