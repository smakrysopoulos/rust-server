import aiohttp
import asyncio
import time

# Define the URL for the request
url = 'http://localhost:5001/build_metadata?version=1.0.3&branch=master&image_name=imagge'

# Function to send a single request and measure the response time
async def send_request(session, url, total_time, count):
    try:
        start_time = time.time()  # Start time
        async with session.get(url) as response:
            status = response.status
            text = await response.text()
            end_time = time.time()  # End time
            response_time = end_time - start_time
            total_time += response_time  # Accumulate total time
            count += 1  # Increment count
            #print(f"Status: {status}, Response: {text}, Response Time: {response_time:.4f}s")
    except Exception as e:
        print(f"Request failed: {e}")
    return total_time, count

# Function to send multiple requests and calculate average response time
async def send_requests(url, n=1000):
    total_time = 0
    count = 0
    async with aiohttp.ClientSession() as session:
        tasks = []
        for _ in range(n):
            tasks.append(send_request(session, url, total_time, count))
        
        # Execute all tasks concurrently
        results = await asyncio.gather(*tasks)

        # Calculate total time and count from the results
        total_time = sum(result[0] for result in results)
        count = sum(result[1] for result in results)

    # Calculate average response time
    if count > 0:
        average_response_time = total_time / count
        print(f"Average Response Time: {average_response_time:.4f}s")
    else:
        print("No requests were completed successfully.")

# Main entry point
if __name__ == "__main__":
    asyncio.run(send_requests(url, 60000))
