import { useEffect, useState } from "react";
import {
  CartesianGrid,
  Legend,
  Line,
  LineChart,
  Tooltip,
  XAxis,
  YAxis,
  ResponsiveContainer,
  BarChart,
  Bar,
  ReferenceLine,
} from "recharts";

interface MonthlyDebit {
  month: string;
  total_debit: number;
  total_credit: number;
  balance: number;
}

export function MonthlyAnalysis() {
  const [data, setData] = useState<MonthlyDebit[]>([]);

  useEffect(() => {
    fetch("http://0.0.0.0:3000/get_monthly_debit")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! Status: ${response.status}`);
        }
        return response.json();
      })
      .then((json) => setData(json))
      .catch((error) => console.error("Error fetching data: ", error));
  }, []);

  return (
    <div className="month_component" style={{ height: "1000px" }}>
      <ResponsiveContainer width="99%" height="30%">
        <LineChart
          data={data}
          margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
        >
          <XAxis dataKey="month" />
          <YAxis />
          <Tooltip />
          <Legend />
          <CartesianGrid strokeDasharray="3 3" />
          <Line
            type="monotone"
            dataKey="total_debit"
            name="Money Debited"
            stroke="#e20404"
          />
          <Line
            type="monotone"
            dataKey="total_credit"
            name="Money Credited"
            stroke="#679A7A"
          />
        </LineChart>
      </ResponsiveContainer>
      <ResponsiveContainer width="100%" height="30%">
        <BarChart
          data={data}
          margin={{ top: 20, right: 30, left: 20, bottom: 5 }}
        >
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="month" />
          <YAxis />
          <Tooltip />
          <Legend />
          <Bar dataKey="total_credit" stackId="a" fill="#679a7a" />
          <Bar dataKey="total_debit" stackId="a" fill="#d33d3d" />
        </BarChart>
      </ResponsiveContainer>
      <ResponsiveContainer width="100%" height="30%">
        <BarChart
          data={data}
          margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
        >
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="month" />
          <YAxis />
          <Tooltip />
          <Legend />
          <ReferenceLine y={0} stroke="#000" />
          <Bar dataKey="balance" fill="#8884d8" name="Monthly Balance" />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
