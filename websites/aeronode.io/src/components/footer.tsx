import { IMAGE_LOGO, LINK_MOONLITGARAGE } from "@/common/constants";
import { Link } from "react-router-dom";
import { useTheme } from '@/components/theme-provider';


export default function Footer() {
    const { theme } = useTheme();
    const isDarkTheme = theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
    
    return (
        <div id="header" className="mb-0">
            <footer className="sticky flex h-16 items-center gap-4 border-dashed border-t bg-background px-4 md:px-6">
                <div className="flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4">
                    <div className="ml-auto mr-auto flex-1 sm:flex-initial p-2">
                        <div className="flex items-center gap-2 text-lg md:text-base font-semibold">
                            <span>A Project By</span>
                            <Link to={LINK_MOONLITGARAGE} className="hover:underline">
                                <img 
                                    src={IMAGE_LOGO} 
                                    alt="Moonlit Garage logo" 
                                    className={`mx-auto h-8 transition-all duration-300 ${isDarkTheme ? 'invert' : ''}`}
                                />
                            </Link>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    );
}