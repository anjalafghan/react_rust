import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    fetch("http://0.0.0.0:3000/get_monthly_analysis")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! Status: ${response.status}`);
        }
        return response.json();
      })
      .then((json) => setData(json))
      .catch((error) => console.log("Error: ", error));
  }, []);

  return (
    <>
      <h1>Hello world {data ? JSON.stringify(data) : "Loading..."}</h1>
    </>
  );
}

export default App;
