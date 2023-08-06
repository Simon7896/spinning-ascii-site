import { ButtonHTMLAttributes } from "react";

type Props = {
    className?: string,
    type?: ButtonHTMLAttributes<HTMLButtonElement>['type'],
    children?: React.ReactNode,
    onClick?: ()=>{}
}

const RectButton = ( props: Props ) => (
    <button
        className={`bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ${props.className}`}
        type={ props.type }
        onClick={ props.onClick }
    >
        { props.children }
    </button>
);

export default RectButton;
