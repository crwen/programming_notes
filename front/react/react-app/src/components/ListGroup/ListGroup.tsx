// import { Fragment } from "react";
import { useState } from "react";
// import styles from './ListGroup.module.css'
import "./ListGroup.css";
// import styled from 'styled-components'

interface ListGroupProps {
  items: String[];
  heading: String;
  onSelectedItem: (items: String) => void;
}

function ListGroup({ items, heading, onSelectedItem }: ListGroupProps) {
  // Hook
  // 1. 异步更新
  // 2. 存储在组件之外
  // 3. 只能在组件顶层使用
  const [selectedIndex, setSelectedIndex] = useState(-1);
  // const handleClick = (event: MouseEvent) => console.log(event);
  // JSX 中只能使用 html 或者其他 react 组件
  return (
    /* <Fragment> */
    <>
      <h1>{heading}</h1>
      {items.length === 0 ? <p>No item found</p> : null}
      {items.length === 0 && <p>No item found</p>}
      <ul className="list-group">
        {items.map((item, index) => (
          <li
            className={
              index === selectedIndex
                ? "list-group-item active"
                : "list-group-item"
            }
            key={index}
            onClick={() => {
              setSelectedIndex(index);
              onSelectedItem(item);
            }}
          >
            {item}
          </li>
        ))}
      </ul>
    </>
    /* </Fragment> */
  );
}

export default ListGroup;
