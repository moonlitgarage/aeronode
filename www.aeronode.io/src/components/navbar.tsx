import { Link } from "react-router-dom";
import { ModeToggle } from "@/components/mode-toggle";
import Logo from "../../public/logo.png";
import { Avatar, AvatarImage } from "./ui/avatar";
import { LINK_DOCS, LINK_GITHUB } from "@/common/constants";

const headerLinks = [
    {
        name: 'about',
        link: '/about'
    },
    {
        name: 'github',
        link: LINK_GITHUB
    },
    {
        name: 'docs',
        link: LINK_DOCS
    },
];

export default function Navbar() {
    return (
        <div id="header">
            <header className="sticky top-0 flex h-16 items-center gap-4 border-dashed border-b bg-background px-4 md:px-6">
                <nav className="flex flex-row items-center gap-5 text-sm">
                    <Link to="/">
                    <div className="cursor-pointer hover:bg-gray-200 rounded-full p-0.5">
                        <Avatar>
                            <AvatarImage src={Logo} alt="logo" width={70} />
                        </Avatar>
                        </div>
                    </Link>
                    {headerLinks.map((link, index) => (
                        <Link key={index} to={link.link} className="hover:underline">{link.name}</Link>
                    ))}
                </nav>
                <div className="flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4">
                    <div className="ml-auto flex-1 sm:flex-initial" />
                    <ModeToggle />
                </div>
            </header>
        </div>
    );
}
