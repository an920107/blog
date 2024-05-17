import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faClock } from '@fortawesome/free-regular-svg-icons';
import { faHashtag } from '@fortawesome/free-solid-svg-icons';
import { Post } from '@/@types/post';
import { format } from 'date-fns';
import React from 'react';

type Props = {}

export default function PostPage({ }: Props) {
  return (
    <>
      <div className="container">
        <h2 className="my-12">文章</h2>
        <div className="grid grid-cols-3 gap-x-8 gap-y-10">
          {
            posts.map((post) => (
              <a key={post.id} href={`/post/${post.id}`} className="font-normal">
                <div className="rounded-xl shadow-md transition-shadow duration-300 hover:shadow-xl image-container">
                  <div className="relative">
                    <img src={post.previewImage} className="rounded-xl" />
                    <div className="image-overlay rounded-xl"></div>
                  </div>
                  <div className="grid grid-cols-1 gap-y-2 p-3">
                    <h3>{post.title}</h3>
                    <p>{post.description}</p>
                    <p className="flex items-center">
                      <FontAwesomeIcon icon={faHashtag} className="h-4 w-4 me-2" />
                      {
                        post.tags.map((tag) => (
                          <span key={tag} className="bg-blue-100 px-2 me-1 rounded-full">{tag}</span>
                        ))
                      }
                    </p>
                    <p className="flex items-center">
                      <FontAwesomeIcon icon={faClock} className="h-4 w-4 me-3" />
                      {format(post.updatedTime, "yyyy-MM-dd")}
                    </p>
                  </div>
                </div>
              </a>
            ))
          }
        </div>
      </div>
    </>
  )
}

const posts: Post[] = [
  {
    id: "1",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://www.alleycat.org/wp-content/uploads/2019/03/FELV-cat.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "2",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://www.alleycat.org/wp-content/uploads/2019/03/FELV-cat.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "3",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://www.alleycat.org/wp-content/uploads/2019/03/FELV-cat.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "4",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://www.alleycat.org/wp-content/uploads/2019/03/FELV-cat.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "5",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://www.alleycat.org/wp-content/uploads/2019/03/FELV-cat.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "6",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://www.alleycat.org/wp-content/uploads/2019/03/FELV-cat.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  }
];