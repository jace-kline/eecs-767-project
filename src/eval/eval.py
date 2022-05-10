import requests
from time import sleep
from timeit import default_timer as timer

URL = "http://localhost"
PORT = 8000
QUERY_ENDPOINT = f"{URL}:{PORT}/api/query/"

def time_call(fn):
    start = timer()
    res = fn()
    return (res, timer() - start)

def query(q, num_results=5, relevant=None, include_strsim=False):
    payload = {
        'query': q,
        'num_results': num_results,
        'relevant': relevant,
        'include_strsim': include_strsim
    }
    res = requests.post(QUERY_ENDPOINT, json=payload)
    return res.json()

def response_time_avg(queries, query_kwargs={}):
    durs = []
    for q in queries:
        res = query(q, **query_kwargs)
        durs.append(res['duration'])
    
    return sum(durs) / len(durs)

def response_time_avg_feedback(queries, query_kwargs={}):
    durs = []
    for q in queries:
        res = query(q, **query_kwargs)
        feedback = [ res['vm_results'][0]['path'], res['vm_results'][2]['path'], res['vm_results'][4]['path'] ]
        res = query(q, relevant=feedback)
        durs.append(res['duration'])
    
    return sum(durs) / len(durs)

def response_time_test():
    queries = ["wolf pig", "little red riding hood", "curious monkey", "aesop fable", "tortoise hare"]

    strsim_avg = response_time_avg(queries, query_kwargs={'include_strsim': True})
    no_strsim_avg = response_time_avg(queries, query_kwargs={'include_strsim': False})
    strsim_avg_feedback = response_time_avg_feedback(queries, query_kwargs={'include_strsim': True})
    no_strsim_avg_feedback = response_time_avg_feedback(queries)

    print(f"Strsim avg = {strsim_avg} s")
    print(f"No strsim avg = {no_strsim_avg} s")
    print(f"Strsim avg (w/ feedback) = {strsim_avg_feedback} s")
    print(f"No strsim avg (w/ feedback) = {no_strsim_avg_feedback} s")


def main():
    res = query('wolf forest grandma', num_results=5)
    print(res)
    # print(res['duration'])
    # response_time_test()

if __name__ == "__main__":
    main()