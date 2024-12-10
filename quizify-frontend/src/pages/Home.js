import React, { useEffect, useState } from 'react';

const Home = () => {
  const [apiStatus, setApiStatus] = useState('Checking...');

  useEffect(() => {
    const apiUrl = '/api'; // Use the proxy path configured in Nginx

    fetch(`${apiUrl}/health`)
      .then((response) => {
        if (response.ok) {
          return response.json(); // Assuming backend returns JSON for health check
        }
        throw new Error('Failed to connect to backend');
      })
      .then((data) => setApiStatus(data.status || 'Healthy'))
      .catch((error) => setApiStatus(error.message));
  }, []);

  return (
    <div className="min-h-screen flex flex-col">
      {/* Navbar */}
      <header className="bg-blue-600 text-white py-4 shadow-lg">
        <div className="container mx-auto flex justify-between items-center px-4">
          <h1 className="text-2xl font-bold">Quizify</h1>
          <nav className="space-x-4">
            <a href="/" className="hover:text-gray-300">Home</a>
            <a href="/about" className="hover:text-gray-300">About</a>
          </nav>
          <button className="bg-white text-blue-600 px-4 py-2 rounded hover:bg-gray-200">
            Log In
          </button>
        </div>
      </header>

      {/* Hero Section */}
      <main className="flex-grow bg-gray-100 flex items-center">
        <div className="container mx-auto px-4 text-center py-16">
          <h2 className="text-4xl font-bold text-blue-600 mb-4">
            Welcome to Quizify
          </h2>
          <p className="text-lg text-gray-700 mb-8">
            {apiStatus}
          </p>
          <div className="space-x-4">
            <a
              href="/admin"
              className="bg-blue-600 text-white px-6 py-3 rounded hover:bg-blue-700 transition"
            >
              Admin Login
            </a>
            <a
              href="/player"
              className="bg-gray-200 text-blue-600 px-6 py-3 rounded hover:bg-gray-300 transition"
            >
              Player Login
            </a>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-blue-600 text-white py-4">
        <div className="container mx-auto text-center">
          <p>&copy; 2024 Quizify. All rights reserved.</p>
          <p>Contact us at support@quizify.com</p>
        </div>
      </footer>
    </div>
  );
};

export default Home;
