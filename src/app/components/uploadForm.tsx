type Props = {
    className?: string,
    children: React.ReactNode,
    action: (data: FormData) => void,
}

const UploadForm = ( props: Props ) => (

        <form 
            method="POST"
            encType="multipart/form-data"
            id="uploadForm"
            className={`m-5 flex flex-row justify-center items-center ${props.className}`}
            action={ props.action }
        >
            <label htmlFor="image">Image File: </label>
            <input className="mx-5" type="file" name="image" id="image" />
            { props.children }
        </form>
);

export default UploadForm;