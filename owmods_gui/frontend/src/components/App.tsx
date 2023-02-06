import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";

const App = () => {
    return (
        <main className="container">
            <header>
                <Nav />
            </header>
            <Tabs />
        </main>
    );
};

export default App;
