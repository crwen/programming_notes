import Message from './Message'
import ListGroup from './components/ListGroup'
import Alert from './components/Alert'
import Button from './components/Button';
import { useState } from 'react';
import './App.css'
import NavBar from './components/NavBar';
import Cart from './components/Cart';
import Form from './components/Form';
import ExpenseForm from './expense-tracker/components/ExpenseForm';
import ExpenseList from './expense-tracker/components/ExpenseList';
import ExpenseFilter from './expense-tracker/components/ExpenseFilter';
import categories from './expense-tracker/categories';
import UserList from './backend/UserList';


function App() {

  const [alertVisible, setAlertVisibility] = useState(false);

  const items = ['New York', 'San Francisco', 'Tokyo', 'London', 'Paris'];
  const handleSelectedItem = (item: String) => {
    console.log(item);
  };

  const onClick = () => {
    setAlertVisibility(!alertVisible)
  }
  const onClose = () => {
    setAlertVisibility(false)
  }

  // const expenses = [
  //   { id: 1, description: 'aaa', amount: 1, category: 'Drink' },
  //   { id: 2, description: 'bbb', amount: 1, category: 'Food' },
  //   { id: 3, description: 'ccc', amount: 1, category: 'Utilities' },
  // ]

  const [expenses, setExpenses] = useState([
    { id: 1, description: 'aaa', amount: 1, category: 'Entertainment' },
    { id: 2, description: 'bbb', amount: 2, category: 'Utilities' },
    { id: 3, description: 'ccc', amount: 3, category: 'Groceries' },
  ])

  const [selectedCategory, setSelectedCategory] = useState('')

  const visibleExpenses = selectedCategory ?
    expenses.filter((expense) => selectedCategory === 'All' || expense.category === selectedCategory) :
    expenses

  const onDelete = (id: number) => {
    setExpenses(expenses.filter((expense) => expense.id !== id))
  }

  const onChangeSelected = (category: string) => {
    setSelectedCategory(category)
  }


  const [cartItems, setCarItems] = useState(['Product1', 'Product2'])

  return (<>
    <div><Message /></div>
    <div><ListGroup items={items} heading='Cities' onSelectedItem={handleSelectedItem} /></div>
    {alertVisible && <Alert onClose={onClose}>Hello <span>world</span>! <Message /></Alert>}
    <div><Button color='danger' onClick={onClick}>Hello</Button></div>

    <NavBar itemsCount={cartItems.length}></NavBar>
    <Cart items={cartItems} onClear={() => setCarItems([])}></Cart>

    <Form></Form>


    <ExpenseForm onAdd={data => {
      console.log(data)
      let maxId = 0
      for (let index = 0; index < expenses.length; index++) {
        maxId = Math.max(maxId, expenses[index].id)
      }
      console.log('---' + maxId)
      setExpenses([...expenses, { ...data, id: maxId + 1 }])
    }}></ExpenseForm>
    <ExpenseFilter categories={categories} onSelectCategory={onChangeSelected}></ExpenseFilter>
    <ExpenseList expenses={visibleExpenses} onDelete={onDelete}></ExpenseList>

    <UserList></UserList>
  </ >
  );
}

export default App;
