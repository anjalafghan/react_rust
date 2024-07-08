import { useState } from "react";

import DatePicker from "react-date-picker";

type ValuePiece = Date | null;

type Value = ValuePiece | [ValuePiece, ValuePiece];

export function MyDatePicker() {
  const [value, onChange] = useState<Value>(new Date());

  return (
    <div>
      <div className="NavigationBar">
        <DatePicker onChange={onChange} value={value} />
      </div>
    </div>
  );
}
