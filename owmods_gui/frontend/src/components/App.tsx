import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";
import { invoke } from "@tauri-apps/api";

// Refresh once to get data
invoke("refresh_local_db");

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
