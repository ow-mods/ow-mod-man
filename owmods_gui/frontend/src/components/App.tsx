import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";

export default () => {
    return (
        <main className="container">
            <header>
                <Nav />
            </header>
            <Tabs />
        </main>
    );
};
