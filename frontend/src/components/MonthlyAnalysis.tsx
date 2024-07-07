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
} from "recharts";

interface MonthlyDebit {
  month: string;
  total_debit: string;
  total_credit: string;
  balance: string;
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
    <div className="month_component">
      <ResponsiveContainer width="99%" height={500}>
        <LineChart
          data={data}
          margin={{ top: 0, right: 0, left: 0, bottom: 0 }}
        >
          <CartesianGrid />
          <XAxis dataKey="month" />
          <YAxis />
          <Tooltip />
          <Legend />

          <Line
            type="monotone"
            dataKey="balance"
            name="Balance Left"
            stroke="#82ca9d"
          />
        </LineChart>
      </ResponsiveContainer>
      <ResponsiveContainer width="99%" height={500}>
        <LineChart
          data={data}
          margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
        >
          <CartesianGrid />
          <XAxis dataKey="month" />
          <YAxis />
          <Tooltip />
          <Legend />

          <Line
            type="monotone"
            dataKey="total_credit"
            name="Money Credited"
            stroke="#82ca9d"
          />
        </LineChart>
      </ResponsiveContainer>
      <ResponsiveContainer width="99%" height={500}>
        <LineChart
          data={data}
          margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
        >
          <CartesianGrid />
          <XAxis dataKey="month" />
          <YAxis />
          <Tooltip />
          <Legend />

          <Line
            type="monotone"
            dataKey="total_debit"
            name="Money Debited"
            stroke="#e20404"
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
