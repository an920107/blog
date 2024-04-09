import React from 'react'

type Props = {}

export default function NotFound({ }: Props) {
    return (
        <div className="flex flex-1 justify-center items-center">
            <div className="flex flex-row gap-6 items-end">
                <h1 className="underline">404</h1>
                <h3 className="border-s-8 border-s-slate-800 ps-4">Not Found.</h3>
            </div>
        </div>
    )
}