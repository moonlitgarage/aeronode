import { Link } from "react-router-dom";
import { ModeToggle } from "@/components/mode-toggle";
import Logo from "../../public/logo.png";
import { LINK_DOCS, LINK_GITHUB } from "@/common/constants";
import { Button } from "./ui/button";
import { useTheme } from '@/components/theme-provider';


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
    const { theme } = useTheme();

  const isDarkTheme = theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);

    return (
        <div id="header">
            <header className="sticky top-0 flex h-16 items-center gap-4 border-dashed border-b bg-background px-4 md:px-6">
            <nav className="flex flex-row items-center gap-5 text-sm">
                    <Link to="/" className="w-12">
                    <div className="cursor-pointer hover:bg-gray-200 p-0.5">
                        <img 
                            src={Logo}
                            alt="logo" 
                            className={`mx-auto transition-all duration-300 ${isDarkTheme ? 'invert' : ''}`}
                            style={{ filter: isDarkTheme ? 'invert(1) hue-rotate(180deg)' : 'none' }}
                        />
                    </div>
                    </Link>
                    {headerLinks.map((link, index) => (
                        <Link key={index} to={link.link} className="hover:underline">
                            <Button variant={"outline"}>
                                {link.name}
                            </Button>
                        </Link>
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
